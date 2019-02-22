use serde::{Deserialize, Serialize};

pub use super::bitfield::*;

use crate::{Vector2, Vector3};
use indexmap::IndexMap;

// todo
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct BaseEvent;

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct BasicAttackData {
    pub target_net_id: u32,
    #[serde(with = "crate::f8")]
    pub extra_time: f32,
    pub missile_next_id: u32,
    pub attack_slot: u8,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct BuffAddGroupEntry {
    pub unit_net_id: u32,
    pub caster_net_id: u32,
    pub buff_slot: u8,
    pub count: u8,
    pub is_hidden: bool,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct BuffRemoveGroupEntry {
    pub unit_net_id: u32,
    pub buff_slot: u8,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct BuffReplaceGroupEntry {
    pub unit_net_id: u32,
    pub caster_net_id: u32,
    pub buff_slot: u8,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct BuffUpdateCountGroupEntry {
    pub unit_net_id: u32,
    pub caster_net_id: u32,
    pub buff_slot: u8,
    pub count: u8,
}

//todo https://github.com/moonshadow565/SiphoningStrike/blob/2410c363a69271af09654fa7773f07f05deedace/SiphoningStrike/Game/Common/CastInfo.cs
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CastInfo {
    pub spell_hash: u32,
    pub spell_net_id: u32,
    pub spell_level: u32,
    pub attack_speed_modifier: u32,
    pub caster_net_id: u32,
    pub missile_net_id: u32,
    pub target_position: Vector3,
    pub target_position_end: Vector3,
    #[serde(with = "crate::vec_u8")]
    pub targets_info: Vec<CastTargetInfo>,
    pub designer_cast_time: f32,
    pub extra_cast_time: f32,
    pub designer_total_time: f32,
    pub cooldown: f32,
    pub start_cast_time: f32,
    pub bitfield: CastInfoBitfield,
    pub spell_slot: u8,
    pub mana_cost: u8,
    pub caster_position: Vector3,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CastTargetInfo {
    pub unit_net_id: u32,
    pub position: Vector3,
    pub hit_result: u8,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct Color {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}

//todo pub struct CompressedWayPoints
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CompressedWaypoint;

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct ConnectionInfo {
    pub client_id: u32,
    pub player_id: u64,
    pub percentage: f32,
    pub eta: f32,
    pub count: i16,
    #[serde(with = "crate::mask_0x7fff")]
    pub ping: u16,
    #[serde(with = "crate::bit_bool")]
    pub ready: bool,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct DeathData {
    pub killer_net_id: u32,
    pub damage_type: u8,
    pub spell_source_type: u8,
    pub death_duration: f32,
    pub become_zombie: bool,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct EventData {
    pub time_stamp: f32,
    pub count: u16,
    // check this
    pub source_net_id: u32,
    pub event: BaseEvent,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct FxCreateGroupEntry {
    pub effect_name_hash: u32,
    pub flags: u16,
    pub target_bone_name_hash: u32,
    pub bone_name_hash: u32,
    #[serde(with = "crate::vec_u8")]
    pub fx_create_data: Vec<FxCreateGroupItem>,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct FxCreateGroupItem {
    pub target_net_id: u32,
    pub net_assigned_id: u32,
    pub bind_net_id: u32,
    pub position_x: u16,
    pub position_y: f32,
    pub position_z: u16,
    pub target_position_x: u16,
    pub target_position_y: f32,
    pub target_position_z: u16,
    pub owner_position_x: u16,
    pub owner_position_y: f32,
    pub owner_position_z: u16,
    pub orientation_vector: Vector3,
    pub time_spent: f32,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct ItemData {
    pub slot: u8,
    pub items_in_slot: u8,
    pub spell_charges: u8,
    pub item_id: u32,
}

#[derive(Clone, Debug)]
pub enum MovementData {
    Normal(MovementDataNormal),
    Stop(MovementDataStop),
    Speed(MovementDataWithSpeed),
    None(i32),
}

impl<'de> serde::Deserialize<'de> for MovementData {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, SeqAccess, Visitor};

        struct MovVisitor;

        impl<'de> Visitor<'de> for MovVisitor {
            type Value = MovementData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("movdata")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let lookahead: u8 = seq
                    .next_element()?
                    .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?;
                Ok(match lookahead {
                    1 => MovementData::Speed(
                        seq.next_element()?
                            .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?,
                    ),
                    2 => MovementData::Normal(
                        seq.next_element()?
                            .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?,
                    ),
                    3 => MovementData::Stop(
                        seq.next_element()?
                            .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?,
                    ),
                    _ => MovementData::None(
                        seq.next_element()?
                            .ok_or_else(|| Error::custom(crate::Error::UnexpectedEof))?,
                    ),
                })
            }
        }

        d.deserialize_seq(MovVisitor)
    }
}

impl serde::Serialize for MovementData {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut s = s.serialize_tuple(2)?;
        match self {
            MovementData::Stop(data) => s.serialize_element(data)?,
            MovementData::Normal(data) => s.serialize_element(data)?,
            MovementData::Speed(data) => s.serialize_element(data)?,
            MovementData::None(data) => s.serialize_element(data)?,
        }
        s.end()
    }
}
// todo implement actual de/serialization
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MovementDataNormal {
    pub teleport_net_id: u32,
    pub has_teleport_id: bool,
    pub teleport_id: u8,
    pub waypoints: Vec<CompressedWaypoint>,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct MovementDataStop {
    pub position: Vector2,
    pub forward: Vector2,
}

// todo implement actual de/serialization
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MovementDataWithSpeed {
    pub teleport_net_id: u32,
    pub has_teleport_id: bool,
    pub teleport_id: u8,
    pub waypoints: Vec<CompressedWaypoint>,
    pub speed_params: SpeedParams,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct NavFlagCircle {
    pub position: Vector2,
    pub radius: f32,
    pub flags: u32,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct PlayerLoadInfo {
    pub player_id: u64,
    pub summoner_level: u16,
    pub summoner_spell1: u32,
    pub summoner_spell2: u32,
    pub is_bot: bool,
    pub team_id: u32,
    pub _pad0: [u8; 28],
    pub _pad1: [u8; 28],
    pub bot_difficulty: i32,
    pub profile_icon_id: i32,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct Talent {
    hash: u32,
    level: u8,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct TooltipVars {
    owner_net_id: u32,
    slot_index: u8,
    values: [f32; 3],
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SpeedParams {
    path_speed_override: f32,
    parabolic_gravity: f32,
    parabolic_start_point: Vector2,
    facing: bool,
    follow_net_id: u32,
    follow_distance: f32,
    follow_back_distance: f32,
    follow_travel_time: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ReplicationData {
    unit_net_id: u32,
    values: IndexMap<i32, IndexMap<i32, u32>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct UpdateLevelPropData {
    #[serde(with = "crate::string_64")]
    string_param0: String,
    float_param0: f32,
    float_param1: f32,
    net_id: u32,
    flags: u32,
    command: u8,
    byte_param0: u8,
    byte_param1: u8,
    byte_param2: u8,
}
