use enet_sys as enet;

use core::{mem, ops, ptr, slice};

use crate::{client::ClientId, packet::KeyCheck};

pub struct PacketData(*mut enet::ENetPacket);

impl ops::Deref for PacketData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts((*self.0).data, (*self.0).dataLength) }
    }
}

impl ops::DerefMut for PacketData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut((*self.0).data, (*self.0).dataLength) }
    }
}

impl Drop for PacketData {
    fn drop(&mut self) {
        unsafe { enet::enet_packet_destroy(self.0) };
    }
}

pub enum Event {
    NoEvent,
    Connected(KeyCheck, *mut enet::ENetPeer),
    Disconnected(ClientId),
    // cid, channel, data
    Packet(ClientId, u8, PacketData),
}

pub struct LENetServer {
    host: *mut enet::ENetHost,
}

impl LENetServer {
    pub fn new(address: enet::ENetAddress) -> Self {
        LENetServer {
            host: unsafe { enet::enet_host_create(&address, 32, 0, 0) },
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
                    Ok(Event::Packet(*cid, channel, PacketData(event.packet)))
                } else if event.channelID != 0
                    || unsafe { (*event.packet).dataLength as usize } < mem::size_of::<KeyCheck>()
                {
                    unsafe { enet::enet_packet_destroy(event.packet) };
                    Ok(Event::NoEvent)
                } else {
                    let packet = unsafe { *(data.as_ptr() as *const _ as *const KeyCheck) };
                    unsafe { enet::enet_packet_destroy(event.packet) };
                    Ok(Event::Connected(packet, event.peer))
                }
            }
            _ => Ok(Event::NoEvent),
        }
    }
}

impl Drop for LENetServer {
    fn drop(&mut self) {
        unsafe { enet::enet_host_destroy(self.host) };
    }
}

pub fn set_peer_data<T>(peer: *mut enet::ENetPeer, data: Option<T>) {
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
