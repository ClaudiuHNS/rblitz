use serde::{Deserialize, Serialize};

use super::packet_id;

#[packet_id(0x0B)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SRemoveItemAns {
    pub slot: u8,
    pub items_in_slit: u8,
}

#[packet_id(0x18)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcUpgradeSpellAns {
    pub slot: u8,
    pub spell_level: u8,
    pub skill_points: u8,
}

#[packet_id(0x44)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CSwapItemAns {
    pub source: u8,
    pub destination: u8,
}

#[packet_id(0x72)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SBuyItemAns {
    pub slot: u8,
    pub item_id: u32,
    pub items_in_slot: u8,
    pub use_on_bought: bool,
}

#[packet_id(0x8D)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SQueryStatusAns {
    pub is_ok: bool,
}

#[packet_id(0xA7)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CUseItemAns {
    pub target_net_id: u32,
}
