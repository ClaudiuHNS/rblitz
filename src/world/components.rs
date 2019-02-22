use specs::{Component, DenseVecStorage, HashMapStorage, VecStorage};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct UnitName(pub String);

impl Component for UnitName {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct SummonerSpells(pub u32, pub u32);

impl Component for SummonerSpells {
    type Storage = HashMapStorage<Self>;
}

#[repr(u32)]
#[derive(serde::Deserialize, Copy, Clone, PartialOrd, PartialEq)]
pub enum Team {
    Order = 100,
    Chaos = 200,
}

impl Component for Team {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Health(f32);

impl Component for Health {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct NetId(u32);

impl NetId {
    #[inline]
    pub fn new_spawned(id: u32) -> Self {
        NetId(((NetNodeId::Spawned as u32) << 24) | (id & 0x00FFFFFF))
    }

    #[inline]
    pub fn new_map(id: u32) -> Self {
        NetId(((NetNodeId::Map as u32) << 24) | (id & 0x00FFFFFF))
    }

    #[inline]
    pub fn id(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn node_id(self) -> NetNodeId {
        NetNodeId::from_trusted(((self.0 & 0xFF000000) >> 24) as u8)
    }
}

impl Component for NetId {
    type Storage = DenseVecStorage<Self>;
}

const NET_NODE_ID_SPAWNED: u8 = 0x40;
const NET_NOTE_ID_MAP: u8 = 0xFF;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum NetNodeId {
    Spawned = NET_NODE_ID_SPAWNED,
    Map = NET_NOTE_ID_MAP,
}

impl NetNodeId {
    fn from_trusted(byte: u8) -> Self {
        match byte {
            NET_NODE_ID_SPAWNED => NetNodeId::Spawned,
            NET_NOTE_ID_MAP => NetNodeId::Map,
            _ => unreachable!(),
        }
    }
}
