use block_modes::BlockMode;
use enet_sys as enet;

use core::{cell::UnsafeCell, mem, ptr::NonNull, slice};

use crate::packet::{game::GamePacket, loading_screen::LoadingScreenPacket, Channel, KeyCheck};

type Blowfish = block_modes::Ecb<blowfish::Blowfish, block_modes::block_padding::ZeroPadding>;
pub type ClientMap = indexmap::IndexMap<crate::client::ClientId, crate::client::Client>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ClientId(pub u32);

pub struct Client {
    pub peer: Option<NonNull<enet::ENetPeer>>,
    blowfish: UnsafeCell<Blowfish>,
}

impl Client {
    pub fn new(key: &[u8]) -> Self {
        Client {
            peer: None,
            blowfish: UnsafeCell::new(Blowfish::new_varkey(key).unwrap()),
        }
    }

    pub fn auth(&mut self, cid: ClientId, mut keycheck: KeyCheck, peer: *mut enet::ENetPeer) {
        let mut check = keycheck.check_id;
        let _ = self.blowfish().decrypt_nopad(&mut check);
        if check != keycheck.player_id.to_le_bytes() {
            return;
        }
        log::debug!("client {:?} authenticated", cid.0);
        crate::lenet_server::set_peer_data(peer, Some(cid));
        self.peer = NonNull::new(peer);

        keycheck.client_id = cid.0;
        self.send_key_check(keycheck);
    }

    // shitty hack cause of how this blowfish works
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

    fn send_data(&self, channel: Channel, data: &mut [u8]) {
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

    pub fn send_game_packet<P: GamePacket>(&self, channel: Channel, sender_net_id: u32, packet: P) {
        if self.peer.is_some() {
            let mut data = Vec::with_capacity(mem::size_of::<P>() + 1 + 4);
            data.push(P::ID);
            data.extend_from_slice(&sender_net_id.to_le_bytes());
            rblitz_packets::to_writer(&packet, &mut data).unwrap();
            self.send_data(channel, data.as_mut_slice());
        }
    }

    pub fn send_loading_screen_packet<P: LoadingScreenPacket>(&self, packet: P) {
        if self.peer.is_some() {
            let mut data = Vec::with_capacity(mem::size_of::<P>() + 1);
            data.push(P::ID);
            rblitz_packets::to_writer(&packet, &mut data).unwrap();
            self.send_data(Channel::LoadingScreen, data.as_mut_slice());
        }
    }

    pub fn send_key_check(&self, mut keycheck: KeyCheck) {
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

impl Drop for Client {
    fn drop(&mut self) {
        if let Some(peer) = self.peer {
            unsafe { enet::enet_peer_disconnect_now(peer.as_ptr(), 0) };
        }
    }
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}
