use block_modes::BlockMode;
use enet_sys as enet;
use indexmap::IndexMap;
use rblitz_packets::packets::loading_screen::{RequestRename, RequestReskin, TeamRosterUpdate};
use specs::{world::Builder, Entity, ReadStorage, World};

use core::{ops, ptr::NonNull};

use crate::{
    config::PlayerConfig,
    error::{Error, Result},
    packet::{
        dispatcher_sys::PacketSender, loading_screen::LoadingScreenPacket, Channel, KeyCheck,
    },
    world::components::{NetId, SummonerSpells, Team, UnitName},
    PLAYER_COUNT_MAX,
};

type Blowfish = block_modes::Ecb<blowfish::Blowfish, block_modes::block_padding::ZeroPadding>;

pub fn init_clients_from_config(world: &mut World, players: Vec<PlayerConfig>) {
    let mut client_map = IndexMap::with_capacity(PLAYER_COUNT_MAX);
    let mut conn_map = IndexMap::with_capacity(PLAYER_COUNT_MAX);
    for (idx, player) in players.into_iter().enumerate().take(PLAYER_COUNT_MAX) {
        let cid = ClientId(idx as u32);
        conn_map.insert(
            cid,
            (
                None,
                Blowfish::new_varkey(&player.key.as_bytes()[..16]).unwrap(),
            ),
        );
        let ent = world
            .create_entity()
            .with(NetId::new_spawned(idx as u32 + 1))
            .with(player.team)
            .with(UnitName(player.champion))
            .with(SummonerSpells(
                player.summoner_spell0,
                player.summoner_spell1,
            ))
            .build();
        client_map.insert(
            cid,
            Client::new(
                ent,
                player.name,
                player.profile_icon,
                player.summoner_level,
                player.player_id,
                player.skin_id,
            ),
        );
    }
    world.add_resource(ClientMap(client_map));
    world.add_resource(ClientConnectionMap(conn_map));
}

pub struct ClientConnectionMap(IndexMap<ClientId, (Option<NonNull<enet::ENetPeer>>, Blowfish)>);

unsafe impl Send for ClientConnectionMap {}
unsafe impl Sync for ClientConnectionMap {}

impl ClientConnectionMap {
    pub fn send_data(&mut self, cid: ClientId, channel: Channel, data: &mut [u8]) {
        if let Some((Some(peer), bf)) = self.0.get_mut(&cid) {
            Self::encrypt(bf, data);
            unsafe {
                enet::enet_peer_send(
                    peer.as_ptr(),
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

    pub fn auth(
        &mut self,
        cid: ClientId,
        player_id: u64,
        keycheck: KeyCheck,
        new_peer: *mut enet::ENetPeer,
    ) -> Result<()> {
        let mut check = keycheck.check_id;
        let (peer, bf) = &mut self.0[&cid];
        if let Some(prev_peer) = peer.take() {
            unsafe { enet::enet_peer_disconnect(prev_peer.as_ptr(), 0) };
        }
        let _ = bf.decrypt_nopad(&mut check);
        if check != keycheck.player_id.to_le_bytes() || player_id != keycheck.player_id {
            return Err(Error::AuthError);
        }
        log::info!("client {:?} authenticated [{:?}]", cid.0, keycheck);
        crate::lenet_server::set_peer_data(new_peer, Some(cid));
        *peer = NonNull::new(new_peer);
        Ok(())
    }

    pub fn disconnect(&mut self, cid: ClientId) {
        if let Some((Some(peer), _)) = self.0.get(&cid) {
            unsafe { enet::enet_peer_disconnect(peer.as_ptr(), 0) }
        }
    }

    #[inline]
    pub fn decrypt(bf: &mut Blowfish, data: &mut [u8]) {
        let nopad_len = data.len() - (data.len() & 0x07);
        bf.decrypt_nopad(&mut data[..nopad_len]).unwrap();
    }

    #[inline]
    pub fn encrypt(bf: &mut Blowfish, data: &mut [u8]) {
        let nopad_len = data.len() - (data.len() & 0x07);
        bf.encrypt_nopad(&mut data[..nopad_len]).unwrap();
    }

    #[inline]
    pub fn decrypt_client(&mut self, cid: ClientId, data: &mut [u8]) {
        Self::decrypt(&mut self.0[&cid].1, data);
    }

    #[inline]
    pub fn encrypt_client(&mut self, cid: ClientId, data: &mut [u8]) {
        Self::encrypt(&mut self.0[&cid].1, data);
    }
}

impl ops::Deref for ClientConnectionMap {
    type Target = IndexMap<ClientId, (Option<NonNull<enet::ENetPeer>>, Blowfish)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ClientConnectionMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub struct ClientMap(IndexMap<ClientId, Client>);

impl ClientMap {
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
}

impl ops::Deref for ClientMap {
    type Target = IndexMap<ClientId, Client>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ClientMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        champion: Entity,
        name: String,
        profile_icon: i32,
        summoner_level: u16,
        player_id: u64,
        skin_id: u32,
    ) -> Self {
        Client {
            name,
            player_id,
            summoner_level,
            profile_icon,
            status: ClientStatus::Disconnected,
            champ_skin_id: skin_id,
            champion,
        }
    }
}
