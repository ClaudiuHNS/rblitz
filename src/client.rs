use block_modes::BlockMode;
use enet_sys as enet;

use core::{cell::UnsafeCell, mem, ops, ptr::NonNull, slice};

use crate::config::PlayerConfig;
use crate::error::{Error, Result};
use crate::packet::{game::GamePacket, loading_screen::LoadingScreenPacket, Channel, KeyCheck};
use rblitz_packets::packets::game::common::PlayerLoadInfo;

type Blowfish = block_modes::Ecb<blowfish::Blowfish, block_modes::block_padding::ZeroPadding>;

pub struct ClientMap(indexmap::IndexMap<ClientId, Client>);

impl ClientMap {
    pub(super) fn send_roster_update(&mut self, cid: ClientId) {
        use rblitz_packets::packets::loading_screen::*;
        let mut roster_update = TeamRosterUpdate::default();
        let (mut order_id, mut chaos_id) = (0, 0);
        for (_, client) in self.iter() {
            match client.team {
                Team::Order => {
                    roster_update.order_player_ids[order_id] = client.player_id;
                    order_id += 1;
                }
                Team::Chaos => {
                    roster_update.chaos_player_ids[chaos_id] = client.player_id;
                    chaos_id += 1;
                }
            }
        }

        roster_update.current_team_size_order = order_id as u32;
        roster_update.current_team_size_chaos = chaos_id as u32;
        roster_update.team_size_order = 6; //roster_update.current_team_size_order;
        roster_update.team_size_chaos = 6; //roster_update.current_team_size_chaos;

        let client = self.get_mut(&cid).unwrap();
        client.send_loading_screen_packet(&roster_update);

        let mut reskin = RequestReskin::default();
        reskin.player_id = client.player_id;
        reskin.skin_id = client.skin_id as u32;
        reskin.name = client.champion.clone();

        let mut rename = RequestRename::default();
        rename.player_id = client.player_id;
        rename.skin_id = client.skin_id as u32;
        rename.name = client.name.clone();

        self.broadcast_loading_screen(reskin);
        self.broadcast_loading_screen(rename);
    }

    fn broadcast_loading_screen<P: LoadingScreenPacket>(&mut self, packet: P) {
        for c in self.values_mut() {
            c.send_loading_screen_packet(&packet);
        }
    }

    pub fn broadcast<P: GamePacket>(&mut self, channel: Channel, sender: u32, packet: &P) {
        for c in self.values_mut() {
            c.send_game_packet(channel, sender, packet);
        }
    }
}

impl From<indexmap::IndexMap<ClientId, Client>> for ClientMap {
    fn from(map: indexmap::IndexMap<ClientId, Client>) -> Self {
        ClientMap(map)
    }
}

impl ops::Deref for ClientMap {
    type Target = indexmap::IndexMap<ClientId, Client>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ClientMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(u32)]
#[derive(serde::Deserialize, Copy, Clone, PartialOrd, PartialEq)]
pub enum Team {
    Order = 100,
    Chaos = 200,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ClientId(pub u32);

pub struct Client {
    pub peer: Option<NonNull<enet::ENetPeer>>,
    blowfish: UnsafeCell<Blowfish>,
    pub name: String,
    pub team: Team,
    pub champion: String,
    pub skin_id: u8,
    pub player_id: u64,
    pub summoner_level: u16,
    pub summoner_spell0: u32,
    pub summoner_spell1: u32,
    pub profile_icon: i32,
}

impl Client {
    pub fn new(config: PlayerConfig) -> Self {
        Client {
            peer: None,
            blowfish: UnsafeCell::new(Blowfish::new_varkey(&config.key.as_bytes()[..16]).unwrap()),
            team: config.team,
            name: config.name,
            champion: config.champion,
            skin_id: config.skin_id,
            player_id: config.player_id,
            summoner_level: config.summoner_level,
            summoner_spell0: config.summoner_spell0,
            summoner_spell1: config.summoner_spell1,
            profile_icon: config.profile_icon,
        }
    }

    pub fn player_load_info(&self) -> PlayerLoadInfo {
        PlayerLoadInfo {
            player_id: self.player_id,
            summoner_level: self.summoner_level,
            summoner_spell1: self.summoner_spell0,
            summoner_spell2: self.summoner_spell1,
            is_bot: false,
            team_id: self.team as u32,
            _pad0: Default::default(),
            _pad1: Default::default(),
            bot_difficulty: 0,
            profile_icon_id: self.profile_icon,
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
        Ok(())
    }

    // FIXME shitty hack cause of how this blowfish works
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn blowfish(&self) -> &mut Blowfish {
        unsafe { &mut *self.blowfish.get() }
    }

    pub fn decrypt(&self, data: &mut [u8]) {
        let nopad_len = data.len() - (data.len() % 8);
        self.blowfish()
            .decrypt_nopad(&mut data[..nopad_len])
            .unwrap();
    }

    fn send_data(&mut self, channel: Channel, data: &mut [u8]) {
        let nopadlen = data.len() - (data.len() % 8);
        let _ = self.blowfish().encrypt_nopad(&mut data[..nopadlen]);
        unsafe {
            enet::enet_peer_send(
                self.peer.unwrap().as_ptr(),
                channel as u8,
                enet::enet_packet_create(
                    data.as_ptr(),
                    data.len(),
                    enet::_ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE,
                ),
            );
        }
    }

    pub fn send_game_packet<P: GamePacket>(
        &mut self,
        channel: Channel,
        sender_net_id: u32,
        packet: &P,
    ) {
        if self.peer.is_some() {
            log::trace!("[SENT] {:?}", packet);
            // FIXME cache the data vector in the client to save up on allocations?
            let mut data = Vec::with_capacity(mem::size_of::<P>() + 1 + 4);
            data.push(P::ID);
            data.extend_from_slice(&sender_net_id.to_le_bytes());
            rblitz_packets::to_writer(packet, &mut data).unwrap();
            self.send_data(channel, data.as_mut_slice());
        }
    }

    pub fn send_loading_screen_packet<P: LoadingScreenPacket>(&mut self, packet: &P) {
        if self.peer.is_some() {
            log::trace!("[SENT] {:?}", packet);
            let mut data = Vec::with_capacity(mem::size_of::<P>() + 1);
            data.push(P::ID);
            rblitz_packets::to_writer(packet, &mut data).unwrap();
            self.send_data(Channel::LoadingScreen, data.as_mut_slice());
        }
    }

    pub fn send_key_check(&mut self, mut keycheck: KeyCheck) {
        unsafe {
            if self.peer.is_some() {
                let data = slice::from_raw_parts_mut(
                    &mut keycheck as *mut _ as *mut u8,
                    mem::size_of::<KeyCheck>(),
                );
                self.send_data(Channel::Handshake, data);
            }
        }
    }
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}
