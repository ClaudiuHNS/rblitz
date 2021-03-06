//! currently this is just a collection of almost all packet handlers, ideally the packet handler
//! should be defined where they make the most sense though

use byteorder::{ReadBytesExt, LE};
use serde::{Deserialize, Serialize};
use shred::{ReadExpect, Resources, SystemData, WriteExpect};
use specs::ReadStorage;

use rblitz_packets::{
    packets::game::{answer::SQueryStatusAns, common::*, request::CQueryStatusReq, *},
    PacketId, Vector2,
};

use crate::{
    client::{ClientId, ClientMap, ClientStatus},
    error::Result,
    packet::{packet_dispatcher_sys::PacketSender, Channel},
    world::components::{NetId, SummonerSpells, Team, UnitName},
};

pub trait PacketHandler<'a> {
    fn handle(
        &self,
        res: &'a Resources,
        cid: ClientId,
        sender_net_id: u32,
        data: &[u8],
    ) -> Result<()>;
}

pub struct PacketHandlerDummy<P: GamePacket>(pub core::marker::PhantomData<P>);

impl<'a, T> PacketHandler<'a> for PacketHandlerDummy<T>
where
    T: PacketHandlerImpl<'a>,
{
    fn handle(
        &self,
        res: &'a Resources,
        cid: ClientId,
        sender_net_id: u32,
        data: &[u8],
    ) -> Result<()> {
        let packet = rblitz_packets::from_bytes::<T>(data)?;
        log::trace!("[RECEIVED] {:?}", packet);
        packet.handle_self(T::Data::fetch(res), cid, sender_net_id)
    }
}

// clean up the super trait requirements
pub trait GamePacket: PacketId + Serialize + Sized + std::fmt::Debug {
    fn to_bytes(&self, sender_net_id: u32) -> Box<[u8]> {
        let mut data = Vec::with_capacity(core::mem::size_of::<Self>() + 1 + 4);
        data.push(Self::ID);
        data.extend_from_slice(&sender_net_id.to_le_bytes());
        rblitz_packets::to_writer(&self, &mut data).unwrap();
        data.into_boxed_slice()
    }
}
impl<T> GamePacket for T where T: PacketId + Serialize + std::fmt::Debug {}

pub trait PacketHandlerImpl<'a>: GamePacket + for<'de> Deserialize<'de> {
    type Data: SystemData<'a>;

    fn handle_self(self, data: Self::Data, cid: ClientId, sender_net_id: u32) -> Result<()>;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct RawGamePacket<'a> {
    pub id: u8,
    pub sender_net_id: u32,
    pub data: &'a [u8],
}

impl<'a> RawGamePacket<'a> {
    pub fn from_slice(mut data: &'a [u8]) -> Result<RawGamePacket<'a>> {
        Ok(RawGamePacket {
            id: data.read_u8()?,
            sender_net_id: data.read_u32::<LE>()?,
            data,
        })
    }
}

impl<'a> PacketHandlerImpl<'a> for CQueryStatusReq {
    type Data = PacketSender<'a>;
    fn handle_self(self, sender: Self::Data, cid: ClientId, _: u32) -> Result<()> {
        sender.single_packet(cid, Channel::Broadcast, 0, &SQueryStatusAns { is_ok: true });
        Ok(())
    }
}

impl<'a> PacketHandlerImpl<'a> for CReconnect {
    type Data = PacketSender<'a>;
    fn handle_self(self, sender: Self::Data, cid: ClientId, _: u32) -> Result<()> {
        sender.single_packet(
            cid,
            Channel::ClientToServer,
            0,
            &SReconnect { client_id: cid.0 },
        );
        Ok(())
    }
}

impl<'a> PacketHandlerImpl<'a> for CSendSelectedObjID {
    type Data = ();
    fn handle_self(self, _: Self::Data, _cid: ClientId, _: u32) -> Result<()> {
        Ok(())
    }
}

impl<'a> PacketHandlerImpl<'a> for CSyncVersion {
    type Data = (
        ReadStorage<'a, SummonerSpells>,
        ReadStorage<'a, Team>,
        ReadExpect<'a, ClientMap>,
        PacketSender<'a>,
    );
    fn handle_self(
        self,
        (summoner_spells, teams, clients, sender): Self::Data,
        cid: ClientId,
        _: u32,
    ) -> Result<()> {
        let mut player_info: [PlayerLoadInfo; 12] = Default::default();
        for (load_info, client) in player_info.iter_mut().zip(clients.values()) {
            let sums = summoner_spells.get(client.champion).unwrap();
            *load_info = PlayerLoadInfo {
                player_id: client.player_id,
                summoner_level: client.summoner_level,
                summoner_spell1: sums.0,
                summoner_spell2: sums.1,
                is_bot: false,
                team_id: *teams.get(client.champion).unwrap() as u32,
                _pad0: Default::default(),
                _pad1: Default::default(),
                bot_difficulty: 0,
                profile_icon_id: client.profile_icon,
            };
        }

        sender.single_packet(
            cid,
            Channel::Broadcast,
            0,
            &SSyncVersion {
                is_version_ok: true,
                map: 8, //todo replace with world.read_resource::<Map>().id
                player_info,
                version_string: self.version,
                map_mode: "ODIN".to_owned(),
            },
        );
        Ok(())
    }
}

impl<'a> PacketHandlerImpl<'a> for CClientReady {
    type Data = (
        ReadStorage<'a, NetId>,
        WriteExpect<'a, ClientMap>,
        PacketSender<'a>,
    );
    fn handle_self(
        self,
        (net_ids, mut clients, sender): Self::Data,
        cid: ClientId,
        _: u32,
    ) -> Result<()> {
        clients.get_mut(&cid).unwrap().status = ClientStatus::Ready;
        if clients.values().all(|c| c.status == ClientStatus::Ready) {
            log::info!("All clients ready, starting game");
            sender.broadcast_all(
                Channel::Broadcast,
                0,
                &SStartGame {
                    tournament_pause_enabled: false,
                },
            );
            for (cid, c) in clients.iter_mut() {
                c.status = ClientStatus::Connected;
                let net_id = net_ids.get(c.champion).unwrap();
                sender.single_packet(
                    *cid,
                    Channel::Broadcast,
                    net_id.id(),
                    &SOnEnterVisibilityClient {
                        entries: Vec::new(),
                        look_at_pos: None,
                        movement_data: MovementData::Stop(MovementDataStop {
                            position: Vector2 { x: 26.0, y: 280.0 },
                            forward: Vector2 { x: 26.0, y: 280.0 },
                        }),
                    },
                );
            }
        }
        Ok(())
    }
}

// reply with spawn packets here, client only replies with CClientReady after a SSpawnEnd
impl<'a> PacketHandlerImpl<'a> for CCharSelected {
    type Data = (
        ReadStorage<'a, SummonerSpells>,
        ReadStorage<'a, Team>,
        ReadStorage<'a, NetId>,
        ReadStorage<'a, UnitName>,
        ReadExpect<'a, ClientMap>,
        PacketSender<'a>,
    );
    fn handle_self(
        self,
        (summoner_spells, teams, net_ids, unit_names, clients, sender): Self::Data,
        cid: ClientId,
        _: u32,
    ) -> Result<()> {
        let mut hero_data: [(SCreateHero, SAvatarInfo); 12] = Default::default();
        for ((cid, client), hero_data) in clients.iter().zip(hero_data.iter_mut()) {
            let net_id = net_ids.get(client.champion).unwrap();
            let sums = summoner_spells.get(client.champion).unwrap();

            *hero_data = (
                SCreateHero {
                    unit_net_id: net_id.id(),
                    client_id: cid.0,
                    net_node_id: net_id.node_id() as u8,
                    skill_level: 0,
                    team_is_order: *teams.get(client.champion).unwrap() == Team::Order,
                    is_bot: false,
                    bot_rank: 0,
                    // FIXME
                    spawn_position_index: cid.0 as u8 % 5,
                    skin_id: client.champ_skin_id,
                    name: client.name.clone(),
                    skin: unit_names.get(client.champion).unwrap().0.clone(),
                },
                SAvatarInfo {
                    summoner_spell_ids: [sums.0, sums.1],
                    level: 1,
                    ..Default::default()
                },
            );
        }

        sender.single_packet(
            cid,
            Channel::Broadcast,
            0,
            &SStartSpawn {
                bot_count_order: 0,
                bot_count_chaos: 0,
            },
        );
        // FIXME make a function for this loop kinda thing in PacketSender?
        for (create, avatar) in hero_data.iter().take(clients.len()) {
            sender.single_packet(cid, Channel::Broadcast, 0, create);
            sender.single_packet(cid, Channel::Broadcast, 0, avatar);
        }
        sender.single_packet(cid, Channel::Broadcast, 0, &SEndSpawn);
        Ok(())
    }
}

// Turns out riot is just horrible and uses some weird interpolation for the loading percentage
// client side which results in completely inaccurate loading progression
impl<'a> PacketHandlerImpl<'a> for CPingLoadInfo {
    type Data = (ReadExpect<'a, ClientMap>, PacketSender<'a>);
    fn handle_self(mut self, (clients, sender): Self::Data, cid: ClientId, _: u32) -> Result<()> {
        let client = clients.get(&cid).unwrap();
        self.connection_info.player_id = client.player_id;
        sender.broadcast_all(
            Channel::Broadcast,
            0,
            &SPingLoadInfo {
                connection_info: self.connection_info,
            },
        );
        Ok(())
    }
}

impl<'a> PacketHandlerImpl<'a> for CExit {
    type Data = WriteExpect<'a, ClientMap>;
    fn handle_self(self, mut clients: Self::Data, cid: ClientId, _: u32) -> Result<()> {
        clients.get_mut(&cid).unwrap().disconnect();
        Ok(())
    }
}

impl<'a> PacketHandlerImpl<'a> for CWorldSendCameraServer {
    type Data = ();
    #[inline]
    fn handle_self(self, _: Self::Data, _cid: ClientId, _: u32) -> Result<()> {
        Ok(())
    }
}

impl<'a> PacketHandlerImpl<'a> for CWorldLockCameraServer {
    type Data = ();
    #[inline]
    fn handle_self(self, _: Self::Data, _cid: ClientId, _: u32) -> Result<()> {
        Ok(())
    }
}
