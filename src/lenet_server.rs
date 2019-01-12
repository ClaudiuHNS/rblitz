use block_modes::BlockMode;
use byteorder::{ReadBytesExt, LE};
use enet_sys as enet;
use indexmap::IndexMap;

use core::{mem, ops, ptr, slice};

type Blowfish = block_modes::Ecb<blowfish::Blowfish, block_modes::block_padding::ZeroPadding>;

type ClientId = u32;

pub struct PacketData(*mut enet::ENetPacket);

impl ops::Deref for PacketData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts((*self.0).data, (*self.0).dataLength) }
    }
}

impl Drop for PacketData {
    fn drop(&mut self) {
        unsafe { enet::enet_packet_destroy(self.0) };
    }
}

pub enum Event {
    NoEvent,
    Connected(ClientId),
    Disconnected(ClientId),
    // cid, channel, data
    Packet(ClientId, u8, PacketData),
}

struct ClientMap {
    peers: IndexMap<ClientId, *mut enet::ENetPeer>,
    blowfish: IndexMap<ClientId, Blowfish>,
}

impl ClientMap {
    fn get_client_blowfish(&mut self, cid: ClientId) -> &mut Blowfish {
        self.blowfish
            .get_mut(&cid)
            .expect("invalid client id for blowfish access")
    }

    #[inline]
    fn insert(&mut self, cid: ClientId, peer: *mut enet::ENetPeer) {
        self.peers.insert(cid, peer);
    }

    #[inline]
    fn remove(&mut self, cid: ClientId) {
        self.peers.remove(&cid);
    }

    fn get_peer(&self, cid: ClientId) -> *mut enet::ENetPeer {
        *self
            .peers
            .get(&cid)
            .expect("invalid client id for peer access")
    }
}

pub struct LENetServer {
    host: *mut enet::ENetHost,
    clients: ClientMap,
}

impl LENetServer {
    pub fn new(address: u32, port: u16, keys: [(ClientId, [u8; 16]); 12]) -> Self {
        let addr = enet::ENetAddress {
            host: address,
            port,
        };

        LENetServer {
            host: unsafe { enet::enet_host_create(&addr, 32, 0, 0) },
            clients: ClientMap {
                peers: IndexMap::with_capacity(12),
                blowfish: keys
                    .iter()
                    .map(|(cid, key)| (*cid, Blowfish::new_varkey(&key[..]).unwrap()))
                    .collect(),
            },
        }
    }

    pub fn service(&mut self, timeout: u32) -> Result<Event, &'static str> {
        let mut event: enet::ENetEvent = unsafe { mem::zeroed() };
        let result = unsafe { enet::enet_host_service(self.host, &mut event, timeout) };
        if result < 0 {
            return Err("something bad happened");
        }
        match event.type_ {
            enet::_ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                set_peer_data::<ClientId>(event.peer, None);
                Ok(Event::NoEvent)
            }
            enet::_ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                if let Some(cid) = peer_data(event.peer) {
                    set_peer_data::<ClientId>(event.peer, None);
                    self.clients.remove(*cid);
                    Ok(Event::Disconnected(*cid))
                } else {
                    Ok(Event::NoEvent)
                }
            }
            enet::_ENetEventType_ENET_EVENT_TYPE_RECEIVE => {
                let data = unsafe {
                    slice::from_raw_parts_mut((*event.packet).data, (*event.packet).dataLength)
                };
                let channel = event.channelID;
                if let Some(cid) = peer_data(event.peer) {
                    let _ = self.clients.get_client_blowfish(*cid).decrypt_nopad(data);
                    Ok(Event::Packet(*cid, channel, PacketData(event.packet)))
                } else if let Some(cid) = self.auth(event.peer, channel, data) {
                    unsafe { enet::enet_packet_destroy(event.packet) };
                    Ok(Event::Connected(cid))
                } else {
                    unsafe {
                        enet::enet_peer_disconnect_now(event.peer, 0);
                        enet::enet_packet_destroy(event.packet);
                    }
                    Ok(Event::NoEvent)
                }
            }
            _ => Ok(Event::NoEvent),
        }
    }

    //none means failure
    fn auth(&mut self, peer: *mut enet::ENetPeer, channel: u8, data: &[u8]) -> Option<u32> {
        if channel != 0 || data.len() < mem::size_of::<PktKeyCheck>() {
            return None;
        }
        let mut packet = unsafe { *(data.as_ptr() as *const PktKeyCheck) };
        let cid = packet.player_id as ClientId;
        if let Some(bf) = self.clients.blowfish.get_mut(&cid) {
            let mut check = packet.check_id;
            let _ = bf.decrypt_nopad(&mut check);
            log::trace!("{:?}", packet);
            if (&check[..]).read_u64::<LE>().unwrap() != packet.player_id {
                return None;
            }

            set_peer_data(peer, Some(cid));
            self.clients.insert(cid, peer);
            packet.client_id = cid;
            let resp_data = unsafe {
                slice::from_raw_parts(
                    &packet as *const _ as *const u8,
                    mem::size_of::<PktKeyCheck>(),
                )
            };
            self.send(cid, 0, resp_data);
            Some(cid)
        } else {
            None
        }
    }

    fn send(&mut self, cid: ClientId, channel: u8, data: &[u8]) {
        let mut data = data.to_owned();
        let _ = self
            .clients
            .get_client_blowfish(cid)
            .encrypt_nopad(data.as_mut_slice());
        unsafe {
            enet::enet_peer_send(
                self.clients.get_peer(cid),
                channel,
                enet::enet_packet_create(
                    data.as_ptr(),
                    data.len(),
                    enet::_ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE,
                ),
            );
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct PktKeyCheck {
    pub action: u8,
    pad: [u8; 3],
    pub client_id: u32,
    pub player_id: u64,
    pub check_id: [u8; 8],
}

fn set_peer_data<T>(peer: *mut enet::ENetPeer, data: Option<T>) {
    unsafe {
        let data_ptr = (*peer).data as *mut T;
        if !data_ptr.is_null() {
            Box::<T>::from_raw(data_ptr);
        }
        (*peer).data = match data {
            Some(data) => Box::into_raw(Box::new(data)) as *mut _,
            None => ptr::null_mut(),
        };
    }
}

fn peer_data<'a, T>(peer: *mut enet::ENetPeer) -> Option<&'a T> {
    unsafe {
        let data_ptr = (*peer).data as *const T;
        if data_ptr.is_null() {
            None
        } else {
            Some(&(*data_ptr))
        }
    }
}
