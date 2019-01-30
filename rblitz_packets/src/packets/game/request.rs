use serde::{Deserialize, Serialize};

use super::packet_id;

#[packet_id(0x09)]
#[derive(Serialize, Copy, Clone, Debug, Default)]
pub struct RemoveItemReq {
    pub slot: u8,
    pub sell: bool,
}

impl<'de> serde::Deserialize<'de> for RemoveItemReq {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bitfield: u8 = Deserialize::deserialize(d)?;
        Ok(RemoveItemReq {
            slot: bitfield & 0x7F,
            sell: bitfield & 0x80 != 0,
        })
    }
}

#[packet_id(0x17)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CQueryStatusReq;

#[packet_id(0x23)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CSwapItemReq {
    pub source: u8,
    pub destination: u8,
}

#[packet_id(0x3E)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CNpcUpgradeSpellReq {
    pub slot: u8,
}

#[packet_id(0x59)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CStatsUpdateReq;

#[packet_id(0x87)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CBuyItemReq {
    pub item_id: u32,
}
