use block_modes::BlockMode;
use enet_sys as enet;
use rblitz_packets::packets::{
    game::server::SWorldSendGameNumber,
    loading_screen::{RequestRename, RequestReskin, TeamRosterUpdate},
};
use specs::{world::Builder, Entity, ReadStorage, World};

use core::{cell::UnsafeCell, mem, ops, ptr::NonNull, slice};

use crate::{
    config::PlayerConfig,
    error::{Error, Result},
    packet::{
        game::GamePacket, loading_screen::LoadingScreenPacket, packet_dispatcher_sys::PacketSender,
        Channel, KeyCheck,
    },
    world::components::{NetId, SummonerSpells, Team, UnitName},
};

type Blowfish = block_modes::Ecb<blowfish::Blowfish, block_modes::block_padding::ZeroPadding>;

pub struct ClientMap {
    clients: indexmap::IndexMap<ClientId, Client>,
}

impl ClientMap {
    pub fn init_from_config(world: &mut World, players: Vec<PlayerConfig>) {
        let clients = players
            .into_iter()
            .take(12)
            .enumerate()
            .map(|(cid, p)| {
                let ent = world
                    .create_entity()
                    .with(NetId::new_spawned(cid as u32 + 1))
                    .with(p.team)
                    .with(UnitName(p.champion))
                    .with(SummonerSpells(p.summoner_spell0, p.summoner_spell1))
                    .build();
                (
                    ClientId(cid as u32),
                    Client::new(
                        &p.key.as_bytes()[..16],
                        ent,
                        p.name,
                        p.profile_icon,
                        p.summoner_level,
                        p.player_id,
                        p.skin_id,
                    ),
                )
            })
            .collect::<indexmap::IndexMap<_, _>>();
        world.add_resource(ClientMap { clients });
    }

    pub(super) fn send_roster_update(
        &self,
        sender: PacketSender<'_>,
        unit_names: &ReadStorage<UnitName>,
        teams: &ReadStorage<Team>,
        cid: ClientId,
    ) {
        let mut roster_update = TeamRosterUpdate::default();
        let (mut order_id, mut chaos_id) = (0, 0);
        for (_, client) in self.iter() {
            match *teams.get(client.champion).unwrap() {
                Team::Order => {
                    roster_update.order_player_ids[order_id] = client.player_id;
                    order_id += 1;
                },
                Team::Chaos => {
                    roster_update.chaos_player_ids[chaos_id] = client.player_id;
                    chaos_id += 1;
                },
            }
        }

        roster_update.current_team_size_order = order_id as u32;
        roster_update.current_team_size_chaos = chaos_id as u32;
        roster_update.team_size_order = 6; //roster_update.current_team_size_order;
        roster_update.team_size_chaos = 6; //roster_update.current_team_size_chaos;
        let packets = self
            .values()
            .map(|client| {
                (
                    RequestReskin {
                        player_id: client.player_id,
                        skin_id: client.champ_skin_id,
                        name: unit_names.get(client.champion).unwrap().0.clone(),
                        ..Default::default()
                    },
                    RequestRename {
                        player_id: client.player_id,
                        skin_id: client.champ_skin_id,
                        name: client.name.clone(),
                        ..Default::default()
                    },
                )
            })
            .collect::<Vec<_>>();
        sender.single(
            cid,
            Channel::LoadingScreen,
            LoadingScreenPacket::to_bytes(&roster_update),
        );
        for (reskin, rename) in packets {
            sender.single(
                cid,
                Channel::LoadingScreen,
                LoadingScreenPacket::to_bytes(&reskin),
            );
            sender.single(
                cid,
                Channel::LoadingScreen,
                LoadingScreenPacket::to_bytes(&rename),
            );
        }
    }

    pub fn broadcast_keycheck(&mut self, cid: ClientId) {
        let packets = self
            .iter()
            .filter(|(cid2, _)| **cid2 != cid)
            .map(|(cid, c)| {
                let mut check_id = c.player_id.to_le_bytes();
                c.blowfish().encrypt_nopad(&mut check_id).unwrap();
                KeyCheck {
                    action: 0,
                    pad: [0, 0, 0],
                    client_id: cid.0,
                    player_id: c.player_id,
                    check_id,
                }
            })
            .collect::<Vec<_>>();
        let client = self.get_mut(&cid).unwrap();
        for packet in packets {
            client.send_key_check(packet);
        }
    }
}

impl From<indexmap::IndexMap<ClientId, Client>> for ClientMap {
    fn from(map: indexmap::IndexMap<ClientId, Client>) -> Self {
        ClientMap { clients: map }
    }
}

impl ops::Deref for ClientMap {
    type Target = indexmap::IndexMap<ClientId, Client>;

    fn deref(&self) -> &Self::Target {
        &self.clients
    }
}

impl ops::DerefMut for ClientMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clients
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum ClientStatus {
    Connected,
    Disconnected,
    Loading,
    Ready,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ClientId(pub u32);

// Make sure that any access to the peer that might mutate is behind a &mut self access for the client,
// otherwise we might end up with data races in enet itself. Also make sure to not access the LEnetServer together with mutable clients
pub struct Client {
    pub peer: Option<NonNull<enet::ENetPeer>>,
    blowfish: UnsafeCell<Blowfish>,
    pub name: String,
    pub player_id: u64,
    pub summoner_level: u16,
    pub profile_icon: i32,
    pub status: ClientStatus,
    pub champ_skin_id: u32,
    pub champion: Entity,
}

impl Client {
    pub fn new(
        key: &[u8],
        champion: Entity,
        name: String,
        profile_icon: i32,
        summoner_level: u16,
        player_id: u64,
        skin_id: u32,
    ) -> Self {
        Client {
            peer: None,
            blowfish: UnsafeCell::new(Blowfish::new_varkey(key).unwrap()),
            name,
            player_id,
            summoner_level,
            profile_icon,
            status: ClientStatus::Disconnected,
            champ_skin_id: skin_id,
            champion,
        }
    }

    pub fn disconnect(&mut self) {
        if let Some(peer) = self.peer.take() {
            unsafe { enet_sys::enet_peer_disconnect(peer.as_ptr(), 0) }
        }
    }

    pub fn auth(
        &mut self,
        cid: ClientId,
        mut keycheck: KeyCheck,
        peer: *mut enet::ENetPeer,
    ) -> Result<()> {
        let mut check = keycheck.check_id;
        let _ = self.blowfish().decrypt_nopad(&mut check);
        if check != keycheck.player_id.to_le_bytes() || self.player_id != keycheck.player_id {
            return Err(Error::AuthError);
        }
        log::info!("client {:?} authenticated [{:?}]", cid.0, keycheck);
        crate::lenet_server::set_peer_data(peer, Some(cid));
        self.peer = NonNull::new(peer);

        keycheck.client_id = cid.0;
        self.send_key_check(keycheck);
        self.send_data(
            // FIXME use PacketSender
            Channel::Broadcast,
            &mut SWorldSendGameNumber { game_id: 12314 }.to_bytes(0),
        );
        Ok(())
    }

    pub fn send_key_check(&mut self, mut keycheck: KeyCheck) {
        log::info!(
            "sending keycheck to player {} {:?}",
            self.player_id,
            keycheck
        );
        unsafe {
            let data = slice::from_raw_parts_mut(
                &mut keycheck as *mut _ as *mut u8,
                mem::size_of::<KeyCheck>(),
            );
            self.send_data(Channel::Handshake, data);
        }
    }

    // FIXME shitty hack cause of how this blowfish works
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn blowfish(&self) -> &mut Blowfish {
        unsafe { &mut *self.blowfish.get() }
    }

    pub fn decrypt(&self, data: &mut [u8]) {
        let nopad_len = data.len() - (data.len() & 0x07);
        self.blowfish()
            .decrypt_nopad(&mut data[..nopad_len])
            .unwrap();
    }

    pub fn encrypt(&self, data: &mut [u8]) {
        let nopadlen = data.len() - (data.len() & 0x07);
        self.blowfish()
            .encrypt_nopad(&mut data[..nopadlen])
            .unwrap();
    }

    pub(super) fn send_data(&mut self, channel: Channel, data: &mut [u8]) {
        if self.peer == None {
            return;
        }
        self.encrypt(data);
        unsafe {
            enet::enet_peer_send(
                self.peer.unwrap().as_ptr(),
                channel as u8,
                enet::enet_packet_create(
                    data.as_ptr(),
                    data.len(),
                    enet::_ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE,
                ),
            )
        };
    }
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}
