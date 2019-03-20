use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CompressedWaypoint {
    pub x: i16,
    pub z: i16,
}

// this is awful garbage
fn read_compressed_waypoints<'de, A>(
    mut seq: A,
    size: u8,
) -> Result<Vec<CompressedWaypoint>, A::Error>
where
    A: serde::de::SeqAccess<'de>,
{
    let mut points = Vec::with_capacity(size as usize);
    let (flags, num_bytes) = if size > 1 {
        let num_bytes = (size - 2) / 4 + 1;
        let mut buf = (0..num_bytes)
            .filter_map(|_| seq.next_element().transpose())
            .collect::<Result<Vec<u8>, _>>()?;
        // cant `rev` the iterator cause it wouldnt change the order
        buf.reverse();
        (bit_vec::BitVec::from_bytes(&buf), num_bytes)
    } else {
        (bit_vec::BitVec::from_elem(8, false), 1)
    };
    let mut last_x: i16 = crate::util::seq_next_elem(&mut seq)?;
    let mut last_z: i16 = crate::util::seq_next_elem(&mut seq)?;
    points.push(CompressedWaypoint {
        x: last_x,
        z: last_z,
    });
    // flagidx should increment up from 0, this is a hack cause of how bitvec works.
    // This might make problems when bitvec is bigger than a byte
    let mut flag_idx = num_bytes as usize * 8 - 1;
    for _ in 1..size {
        if flags[flag_idx] {
            last_x += crate::util::seq_next_elem::<_, i8>(&mut seq)? as i16;
        } else {
            last_x = crate::util::seq_next_elem(&mut seq)?;
        }
        flag_idx -= 1;
        if flags[flag_idx] {
            last_z += crate::util::seq_next_elem::<_, i8>(&mut seq)? as i16;
        } else {
            last_z = crate::util::seq_next_elem(&mut seq)?;
        }
        flag_idx = flag_idx.wrapping_sub(1);
        points.push(CompressedWaypoint {
            x: last_x,
            z: last_z,
        });
    }
    Ok(points)
}

fn write_compressed_waypoints<A>(
    seq: &mut A,
    waypoints: &[CompressedWaypoint],
) -> Result<(), A::Error>
where
    A: serde::ser::SerializeSeq,
{
    assert!(!waypoints.is_empty());
    let flag_array_len = (waypoints.len().saturating_sub(2)) / 4 + 1;
    let mut flags = bit_vec::BitVec::from_elem(flag_array_len * 8, false);

    let mut flag_idx = flags.len() - 1;
    for i in 1..waypoints.len() {
        let relative_x = waypoints[i].x - waypoints[i - 1].x;
        flags.set(
            flag_idx,
            relative_x <= i8::max_value() as i16 && relative_x >= i8::min_value() as i16,
        );
        flag_idx -= 1;

        let relative_z = waypoints[i].z - waypoints[i - 1].z;
        flags.set(
            flag_idx,
            relative_z <= i8::max_value() as i16 && relative_z >= i8::min_value() as i16,
        );
        flag_idx = flag_idx.wrapping_sub(1);
    }
    let mut flag_bytes = flags.to_bytes();
    flag_bytes.reverse();
    seq.serialize_element(&flag_bytes);
    seq.serialize_element(&waypoints[0].x)?;
    seq.serialize_element(&waypoints[0].z)?;

    let mut flag_idx = flags.len() - 1;
    for i in 1..waypoints.len() {
        if flags[flag_idx] {
            seq.serialize_element(&((waypoints[i].x - waypoints[i - 1].x) as i8))?;
        } else {
            seq.serialize_element(&waypoints[i].x)?;
        }
        flag_idx -= 1;
        if flags[flag_idx] {
            seq.serialize_element(&((waypoints[i].z - waypoints[i - 1].z) as i8))?;
        } else {
            seq.serialize_element(&waypoints[i].z)?;
        }
        flag_idx = flag_idx.wrapping_sub(1);
    }
    Ok(())
}

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
        use serde::de::{SeqAccess, Visitor};

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
                let lookahead: u8 = crate::util::seq_next_elem(&mut seq)?;
                Ok(match lookahead {
                    1 => MovementData::Speed(crate::util::seq_next_elem(&mut seq)?),
                    2 => MovementData::Normal(crate::util::seq_next_elem(&mut seq)?),
                    3 => MovementData::Stop(crate::util::seq_next_elem(&mut seq)?),
                    _ => MovementData::None(crate::util::seq_next_elem(&mut seq)?),
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
        let mut s = s.serialize_tuple(1)?;
        match self {
            MovementData::Stop(data) => s.serialize_element(data)?,
            MovementData::Normal(data) => s.serialize_element(data)?,
            MovementData::Speed(data) => s.serialize_element(data)?,
            MovementData::None(data) => s.serialize_element(data)?,
        }
        s.end()
    }
}

#[derive(Clone, Debug, Default)]
pub struct MovementDataNormal {
    pub teleport_id: Option<u8>,
    pub teleport_net_id: Option<u32>,
    pub waypoints: Vec<CompressedWaypoint>,
}

impl<'de> serde::Deserialize<'de> for MovementDataNormal {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{SeqAccess, Visitor};

        struct MovNormalVisitor;

        impl<'de> Visitor<'de> for MovNormalVisitor {
            type Value = MovementDataNormal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("movnormaldata")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let has_teleport_id = (crate::util::seq_next_elem::<_, u16>(&mut seq)? & 1) != 0;
                let size = (crate::util::seq_next_elem::<_, u16>(&mut seq)? & 0x7F) as u8;
                let mut teleport_net_id: Option<u32> = None;
                let mut teleport_id: Option<u8> = None;
                let waypoints = if size != 0 {
                    teleport_net_id = Some(crate::util::seq_next_elem(&mut seq)?);
                    if has_teleport_id {
                        teleport_id = Some(crate::util::seq_next_elem(&mut seq)?);
                    }
                    read_compressed_waypoints(seq, size)?
                } else {
                    Vec::new()
                };
                Ok(MovementDataNormal {
                    teleport_net_id,
                    teleport_id,
                    waypoints,
                })
            }
        }

        d.deserialize_seq(MovNormalVisitor)
    }
}

impl serde::Serialize for MovementDataNormal {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut s = s.serialize_seq(None)?;

        s.serialize_element(&((self.teleport_id.is_some() as u16) << 1))?;
        s.serialize_element(&((self.waypoints.len() as u16) & 0x7F))?;
        if !self.waypoints.is_empty() {
            s.serialize_element(&self.teleport_net_id.unwrap())?;
            if let Some(tp_id) = self.teleport_id {
                s.serialize_element(&tp_id)?;
            }
            write_compressed_waypoints(&mut s, &self.waypoints)?;
        }
        s.end()
    }
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

#[derive(Deserialize_repr, Serialize_repr, Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum OrderType {
    Hold = 1,
    Move = 2,
    AttackMove = 7,
    Stop = 10,
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
