use serde::{Deserialize, Serialize};

use super::bitfield::{
    CharSpawnPetBitfield, ShieldProperties, SpawnMinionBitfield, SpellSlotBitfield,
};
use super::packet_id;
use crate::packets::game::common::*;
use crate::{Vector2, Vector3};
use indexmap::IndexMap;

#[packet_id(0x02)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SDisplayLocalizedTutorialChatText {
    #[serde(with = "crate::string_null")]
    pub message: String,
}

#[packet_id(0x03)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SBarrackSpawnUnit {
    pub unit_net_id: u32,
    pub unit_net_node_id: u8,
    pub wave_count: u8,
    pub minion_type: u8,
    pub damage_bonus: u16,
    pub health_bonus: u16,
}

#[packet_id(0x04)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSwitchNexusesToOnIdleParticles;

#[packet_id(0x06)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetCircularMovementRestriction {
    pub center: Vector3,
    pub radius: f32,
    pub restrict_camera: bool,
}

#[packet_id(0x07)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SUpdateGoldRedirectTarget {
    pub target_net_id: u32,
}

#[packet_id(0x0A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SResumePacket {
    pub client_id: u32,
    pub delayed: bool,
}

#[packet_id(0x0D)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SBasicAttack {
    pub basic_attack_data: BasicAttackData,
}

#[packet_id(0x0E)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SRefreshObjectiveText {
    #[serde(with = "crate::string_null")]
    pub text_id: String,
}

#[packet_id(0x0F)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SCloseShop;

#[packet_id(0x10)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SReconnect {
    pub client_id: u32,
}

#[packet_id(0x11)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SUnitAddExp {
    pub target_net_id: u32,
    pub amount: f32,
}

#[packet_id(0x12)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SEndSpawn;

#[packet_id(0x13)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetFrequency {
    pub new_frequency: f32,
}

#[packet_id(0x14)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHighlightTitanBarElement {
    pub element_type: u8,
}

#[packet_id(0x15)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SBotAi {
    #[serde(with = "crate::string_64")]
    pub ai_name: String,
    #[serde(with = "crate::string_64")]
    pub ai_strategy: String,
    #[serde(with = "crate::string_64")]
    pub ai_behaviour: String,
    #[serde(with = "crate::string_64")]
    pub ai_task: String,
    #[serde(with = "crate::string_64")]
    pub state_0: String,
    #[serde(with = "crate::string_64")]
    pub state_1: String,
    #[serde(with = "crate::string_64")]
    pub state_2: String,
}

#[packet_id(0x16)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct STeamSurrenderCountDown {
    pub time_remaining: f32,
}

#[packet_id(0x1A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SChangeSlotSpellType {
    pub spell_slot: SpellSlotBitfield,
    pub targeting_type: u8,
}

#[packet_id(0x1B)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SNpcMessageToClient {
    pub target_net_id: u32,
    pub bubble_delay: f32,
    pub slot_number: i32,
    pub is_error: bool,
    pub color_index: u8,
    #[serde(with = "crate::string_null")]
    pub message: String,
}

#[packet_id(0x1C)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SDisplayFloatingText {
    pub target_net_id: u32,
    pub floating_text_type: u8,
    pub param: i32,
    #[serde(with = "crate::string_128")]
    pub message: String,
}

#[packet_id(0x1D)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SBasicAttackPos {
    pub basic_attack_data: BasicAttackData,
    pub position: Vector2,
}

#[packet_id(0x1E)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcForceDeath;

#[packet_id(0x1F)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcBuffUpdateCount {
    pub buff_slot: u8,
    pub count: u8,
    pub duration: f32,
    pub running_time: f32,
    pub caster_net_id: u32,
}

#[packet_id(0x21)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SNpcBuffReplaceGroup {
    pub running_time: f32,
    pub duration: f32,
    #[serde(with = "crate::vec_u8")]
    pub entries: Vec<BuffReplaceGroupEntry>,
}

#[packet_id(0x22)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcSetAutocast {
    pub slot: u8,
}

#[packet_id(0x24)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SNpcDeathEventHistory {
    pub killer_net_id: u32,
    pub time_window: f32,
    pub killer_event_source_type: u32,
    pub _buffer_size: u32,
    #[serde(with = "crate::vec_u32")]
    pub events: Vec<EventData>,
}

#[packet_id(0x25)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SUnitAddGold {
    pub target_net_id: u32,
    pub source_net_id: u32,
    pub gold_amount: f32,
}

#[packet_id(0x26)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAddUnitPerceptionBubble {
    pub perception_bubble_type: u32,
    pub client_net_id: u32,
    pub radius: f32,
    pub unit_net_id: u32,
    pub time_to_live: f32,
    pub bubble_id: u32,
    pub flags: u32,
}

#[packet_id(0x27)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SMoveCameraToPoint {
    pub start_from_current_position: bool,
    pub start_position: Vector3,
    pub target_position: Vector3,
    pub travel_time: f32,
}

#[packet_id(0x28)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SLineMissileHitList {
    #[serde(with = "crate::vec_u16")]
    pub target_net_ids: Vec<u32>,
}

#[packet_id(0x29)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SMuteVolumeCategory {
    pub volume_category: u8,
    pub mute: bool,
}

#[packet_id(0x2A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SServerTick {
    pub delta: f32,
}

#[packet_id(0x2B)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SStopAnimation {
    pub fade: bool,
    pub ignore_lock: bool,
    pub stop_all: bool,
}

impl<'de> serde::Deserialize<'de> for SStopAnimation {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let var: u8 = serde::Deserialize::deserialize(d)?;
        Ok(SStopAnimation {
            fade: var & 0x1 != 0,
            ignore_lock: var & 0x2 != 0,
            stop_all: var & 0x4 != 0,
        })
    }
}

impl serde::Serialize for SStopAnimation {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let byte = ((self.fade as u8) << 1)
            | ((self.ignore_lock as u8) << 2)
            | ((self.stop_all as u8) << 4);
        s.serialize_u8(byte)
    }
}

#[packet_id(0x2C)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAvatarInfo {
    pub item_ids: [u32; 30],
    pub summoner_spell_ids: [u32; 2],
    pub talents: [Talent; 30],
    pub level: u8,
}

#[packet_id(0x2D)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SDampenerSwitch {
    pub duration: u16,
    pub state: bool,
}

impl<'de> serde::Deserialize<'de> for SDampenerSwitch {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let var: u16 = serde::Deserialize::deserialize(d)?;
        Ok(SDampenerSwitch {
            duration: var & 0x7FFF,
            state: var & 0x8000 != 0,
        })
    }
}

impl serde::Serialize for SDampenerSwitch {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut short = self.duration;
        if self.state {
            short |= 0x8000;
        }
        s.serialize_u16(short)
    }
}

#[packet_id(0x2E)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SWorldSendCameraServerAck {
    pub sync_id: u8,
}

#[packet_id(0x2F)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SModifyDebugCircleRadius {
    pub circle_id: u32,
    pub radius: f32,
}

#[packet_id(0x31)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHeroReincarnateAlive {
    pub position: Vector3,
}

#[packet_id(0x32)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcBuffReplace {
    pub running_time: f32,
    pub duration: f32,
    pub num_in_group: u8,
    pub caster_net_id: u32,
}

#[packet_id(0x33)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SPause {
    pub client_id: u32,
    pub pause_time_remaining: u32,
    pub tournament_pause: bool,
}

#[packet_id(0x34)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetFadeOutPop {
    pub stack_id: u16,
}

#[packet_id(0x35)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SChangeSlotSpellName {
    pub spell_slot: SpellSlotBitfield,
    #[serde(with = "crate::string_null")]
    pub spellname: String,
}

#[packet_id(0x36)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SChangeSlotSpellIcon {
    pub spell_slot: SpellSlotBitfield,
    pub icon_index: u8,
}

#[packet_id(0x37)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SChangeSpellOffsetTarget {
    pub spell_slot: SpellSlotBitfield,
    pub target_net_id: u32,
}

#[packet_id(0x38)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SRemovePerceptionBubble {
    pub bubble_id: u32,
}

#[packet_id(0x39)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SNpcInstantStopAttack {
    pub keep_animating: bool,
    pub force_spell_cast: bool,
    pub force_stop: bool,
    pub avatar_spell: bool,
    pub destroy_missile: bool,
}

impl<'de> serde::Deserialize<'de> for SNpcInstantStopAttack {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let var: u8 = serde::Deserialize::deserialize(d)?;
        Ok(SNpcInstantStopAttack {
            keep_animating: var & 0x01 != 0,
            force_spell_cast: var & 0x02 != 0,
            force_stop: var & 0x04 != 0,
            avatar_spell: var & 0x08 != 0,
            destroy_missile: var & 0x10 != 0,
        })
    }
}

impl serde::Serialize for SNpcInstantStopAttack {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let byte = ((self.keep_animating as u8) << 1)
            | ((self.force_spell_cast as u8) << 2)
            | ((self.force_stop as u8) << 3)
            | ((self.avatar_spell as u8) << 4)
            | ((self.destroy_missile as u8) << 5);
        s.serialize_u8(byte)
    }
}

#[packet_id(0x3A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnLeaveLocalVisiblityClient;

#[packet_id(0x3B)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SShowObjectiveText {
    #[serde(with = "crate::string_null")]
    pub text_id: String,
}

#[packet_id(0x3C)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SCharSpawnPet {
    pub unit_net_id: u32,
    pub unit_net_node_id: u8,
    pub position: Vector3,
    pub cast_spell_level_plus_one: i32,
    pub duration: f32,
    pub damage_bonus: i32,
    pub health_bonus: i32,
    #[serde(with = "crate::string_32")]
    pub name: String,
    #[serde(with = "crate::string_32")]
    pub skin: String,
    pub skin_id: i32,
    #[serde(with = "crate::string_64")]
    pub buff_name: String,
    pub clone_net_id: u32,
    pub bitfield: CharSpawnPetBitfield,
    #[serde(with = "crate::string_32")]
    pub ai_script: String,
    pub show_minimap_icon: bool,
}

#[packet_id(0x3D)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SFxKill {
    pub unknown_net_id: u32,
}

#[packet_id(0x40)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct STurretCreateTurret;

#[packet_id(0x41)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SMissileReplication {
    pub position: Vector3,
    pub caster_position: Vector3,
    pub direction: Vector3,
    pub velocity: Vector3,
    pub start_point: Vector3,
    pub end_point: Vector3,
    pub unit_position: Vector3,
    pub speed: f32,
    pub life_percentage: f32,
    pub bounced: u8,
    pub cast_info: CastInfo,
}

#[packet_id(0x42)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SResetForSlowLoader;

#[packet_id(0x43)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHighlightHUDElement {
    pub element_type: u8,
    pub element_number: u8,
}

#[packet_id(0x45)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcLevelUp {
    pub level: u8,
    pub available_points: u8,
}

#[packet_id(0x46)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SMapPing {
    pub position: Vector3,
    pub target_net_id: u32,
    pub source_net_id: u32,
    pub bitfield: MapPingBitfield,
}

#[packet_id(0x47)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SWriteNavFlags {
    pub sync_id: i32,
    #[serde(with = "crate::vec_u16")]
    pub nav_flag_cicles: Vec<NavFlagCircle>,
}

#[packet_id(0x48)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SPlayEmote {
    pub emote_id: u32,
}

#[packet_id(0x49)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SReconnectDone;

#[packet_id(0x4A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnEventWorld;

#[packet_id(0x4B)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHeroStats {
    //FIXME: just ignore this useless packet?
}

#[packet_id(0x4D)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHeroReincarnate {
    pub position: Vector3,
}

#[packet_id(0x4F)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SCreateHero {
    pub unit_net_id: u32,
    pub client_id: u32,
    pub net_node_id: u8,
    pub skill_level: u8,
    pub team_is_order: bool,
    pub is_bot: bool,
    pub bot_rank: u8,
    pub spawn_position_index: u8,
    pub skin_id: u32,
    #[serde(with = "crate::string_40")]
    pub name: String,
    #[serde(with = "crate::string_40")]
    pub skin: String,
}

#[packet_id(0x52)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SToggleUIHighlight {
    pub element_id: u8,
    pub element_type: u8,
    pub element_number: u8,
    pub element_sub_category: u8,
    pub enabled: bool,
}

#[packet_id(0x53)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SFaceDirection {
    pub direction: Vector3,
}

#[packet_id(0x54)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnLeaveVisibilityClient;

#[packet_id(0x56)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetItem {
    pub slot: u8,
    pub item_id: u32,
    pub items_in_slot: u8,
    pub spell_charges: u8,
}

#[packet_id(0x57)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SSyncVersion {
    pub is_version_ok: bool,
    pub map: i32,
    pub player_info: [PlayerLoadInfo; 12],
    #[serde(with = "crate::string_256")]
    pub version_string: String,
    #[serde(with = "crate::string_128")]
    pub map_mode: String,
}

#[packet_id(0x58)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SHandleTipUpdate {
    #[serde(with = "crate::string_128")]
    pub tip_name: String,
    #[serde(with = "crate::string_128")]
    pub tip_other: String,
    #[serde(with = "crate::string_128")]
    pub tip_image_path: String,
    pub tip_command: u8,
    pub tip_id: u32,
}

#[packet_id(0x5B)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SRemoveDebugCircle {
    pub debug_id: i32,
}

#[packet_id(0x5C)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SCreateUnitHighlight {
    pub unit_net_id: u32,
}

#[packet_id(0x5D)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SDestroyClientMissile;

#[packet_id(0x5E)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SLevelUpSpell {
    pub spell_slot: u32,
}

#[packet_id(0x5F)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SStartGame {
    pub tournament_pause_enabled: bool,
}

#[packet_id(0x61)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcHeroDie {
    pub death_data: DeathData,
}

#[packet_id(0x62)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SFadeOutMainSFX {
    pub fade_time: f32,
}

#[packet_id(0x63)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SUserMessageStart;

#[packet_id(0x64)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SWaypointGroup {
    pub sync_id: i32,
    #[serde(with = "crate::vec_u16")]
    pub movements: Vec<MovementDataNormal>,
}

#[packet_id(0x65)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SStartSpawn {
    pub bot_count_order: u8,
    pub bot_count_chaos: u8,
}

#[packet_id(0x66)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SCreateNeutral {
    pub unit_net_id: u32,
    pub unit_net_node_id: u8,
    pub position: Vector3,
    pub group_position: Vector3,
    pub face_direction_position: Vector3,
    #[serde(with = "crate::string_64")]
    pub name: String,
    #[serde(with = "crate::string_64")]
    pub skin_name: String,
    #[serde(with = "crate::string_64")]
    pub unique_name: String,
    #[serde(with = "crate::string_64")]
    pub minimap_icon: String,
    pub team_id: u32,
    pub damage_bonus: i32,
    pub health_bonus: i32,
    pub roam_state: i32,
    pub group_number: i32,
    pub behavior_tree: bool,
}

#[packet_id(0x67)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SWaypointGroupWithSpeed {
    pub sync_id: i32,
    #[serde(with = "crate::vec_u16")]
    pub movements: Vec<MovementDataWithSpeed>,
}

#[packet_id(0x68)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SUnitApplyDamage {
    pub damage_result_type: u8,
    //pub has_attack_sound: bool, //bitflag with damage result type
    pub target_net_id: u32,
    pub source_net_id: u32,
    pub damage: f32,
}

#[packet_id(0x69)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SModifyShield {
    pub shield_properties: ShieldProperties,
    pub amount: f32,
}

#[packet_id(0x6A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SPopCharacterData {
    pub pop_id: u32,
}

#[packet_id(0x6B)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SNpcBuffAddGroup {
    pub buff_type: u8,
    pub buff_name_hash: u32,
    pub running_time: f32,
    pub duration: f32,
    #[serde(with = "crate::vec_u8")]
    pub entries: Vec<BuffAddGroupEntry>,
}

#[packet_id(0x6C)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAiTargetSelection {
    pub target_net_ids: [u32; 5],
}

#[packet_id(0x6D)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAiTarget {
    pub target_net_id: u32,
}

//todo de/serialization
#[packet_id(0x6E)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SSetAnimStates {
    pub overrides: IndexMap<String, String>,
}

#[packet_id(0x6F)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SChainMissileSync {
    pub target_count: i32,
    pub owner_network_id: u32,
    //todo fix
    pub target_net_ids: [u32; 32],
}

#[packet_id(0x71)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SMissileReplicationChainMissile;

#[packet_id(0x73)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetSpellData {
    pub unit_net_id: u32,
    pub spell_name_hash: u32,
    pub spell_slot: u8,
}

#[packet_id(0x74)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SPauseAnimation {
    pub pause: bool,
}

#[packet_id(0x75)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SNpcIssueOrderReq {
    pub order_type: u8,
    pub position: Vector3,
    pub target_net_id: u32,
    #[serde(default)]
    pub movement_data: MovementDataNormal,
}

#[packet_id(0x76)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SCameraBehavior {
    pub position: Vector3,
}

#[packet_id(0x77)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAnimatedBuildingSetCurrentSkin {
    pub team_id: u8,
    pub skin_id: u32,
}

#[packet_id(0x78)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SConnected {
    pub client_id: u32,
}

#[packet_id(0x79)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSyncSimTimeFinal {
    pub time_last_client: f32,
    pub time_rtt_last_overhead: f32,
    pub time_convergence: f32,
}

#[packet_id(0x7A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SWaypointAcc {
    pub sync_id: i32,
    pub teleport_count: u8,
}

#[packet_id(0x7B)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAddPosPerceptionBubble {
    pub perception_bubble_type: u32,
    pub client_net_id: u32,
    pub radius: f32,
    pub position: Vector3,
    pub time_to_live: f32,
    pub bubble_id: u32,
    pub flags: u32,
}

#[packet_id(0x7C)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SLockCamera {
    pub lock: bool,
}

#[packet_id(0x7D)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SPlayVOAudioEvent {
    #[serde(with = "crate::string_64")]
    pub folder_name: String,
    #[serde(with = "crate::string_64")]
    pub event_id: String,
    pub callback_type: u8,
    pub audio_event_net_id: u32,
}

#[packet_id(0x7E)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SAiCommand {
    #[serde(with = "crate::string_null")]
    pub command: String,
}

#[packet_id(0x7F)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcBuffRemove {
    pub buff_slot: u8,
    pub buff_name_hash: u32,
}

#[packet_id(0x80)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SSpawnMinion {
    pub unit_net_id: u32,
    pub unit_net_node_id: u8,
    pub position: Vector3,
    pub skin_id: u32,
    pub clone_net_id: u32,
    pub team_id: u32,
    pub visibility_size: f32,
    pub bitfield: SpawnMinionBitfield,
    #[serde(with = "crate::string_64")]
    pub name: String,
    #[serde(with = "crate::string_64")]
    pub skin_name: String,
}
#[packet_id(0x82)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SToggleFoW;

#[packet_id(0x83)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SToolTipVars {
    #[serde(with = "crate::vec_u16")]
    pub tooltip_vars_list: Vec<TooltipVars>,
}

#[packet_id(0x84)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SUnitApplyHeal {
    pub max_hp: f32,
    pub heal: f32,
}

#[packet_id(0x85)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SGlobalCombatMessage {
    pub message_type: u32,
    pub object_name_net_id: u32,
}

#[packet_id(0x86)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SWorldLockCameraServer {
    pub locked: bool,
    pub client_id: u32,
}

#[packet_id(0x88)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SWaypointListHeroWithSpeed {
    pub sync_id: i32,
    pub speed_params: SpeedParams,
    pub waypoitns: Vec<Vector2>,
}

#[packet_id(0x89)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetInputLockingFlag {
    pub input_locking_flags: u32,
    pub value: bool,
}

#[packet_id(0x8A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SCharSetCooldown {
    pub spell_slot: SpellSlotBitfield,
    pub cooldown: f32,
}

#[packet_id(0x8B)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SCharCancelTargetingReticle {
    pub spell_slot: SpellSlotBitfield,
}

#[packet_id(0x8C)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SFxCreateGroup {
    #[serde(with = "crate::vec_u8")]
    pub entries: Vec<FxCreateGroupEntry>,
}

#[packet_id(0x8E)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SBuildingDie {
    pub attacker_net_id: u32,
}

#[packet_id(0x90)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SHandleQuestUpdate {
    #[serde(with = "crate::string_128")]
    pub objective: String,
    #[serde(with = "crate::string_128")]
    pub tooltip: String,
    #[serde(with = "crate::string_128")]
    pub reward: String,
    pub quest_type: u8,
    pub command: u8,
    pub handle_rollovers: bool,
    pub quest_id: u32,
}

#[packet_id(0x95)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SServerGameSettings {
    pub fow_local_culling: bool,
    pub for_broadcast_everything: bool,
}

#[packet_id(0x96)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SModifyDebugCircleColor {
    pub object_id: u32,
    pub color: Color,
}

#[packet_id(0x98)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SWorldSendGameNumber {
    pub game_id: u64,
}

#[packet_id(0x99)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SChangeParColorOverride {
    pub unit_net_id: u32,
    pub enabled: bool,
    pub bar_color: Color,
    pub fade_color: Color,
}

#[packet_id(0x9B)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SNpcBuffRemoveGroup {
    pub buff_name_hash: u32,
    #[serde(with = "crate::vec_u8")]
    pub entries: Vec<BuffRemoveGroupEntry>,
}

#[packet_id(0x9C)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct STurretFire;

#[packet_id(0x9D)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SPingLoadInfo {
    pub connection_info: ConnectionInfo,
}

#[packet_id(0x9E)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SChangeCharacterVoice {
    pub is_reset: bool,
    #[serde(with = "crate::string_null")]
    pub voice_override: String,
}

#[packet_id(0x9F)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SChangeCharacterData {
    pub stack_id: u32,
    pub use_spells: bool,
    #[serde(with = "crate::string_null")]
    pub skin_name: String,
}

#[packet_id(0xA0)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SExit {
    pub client_id: u32,
}

#[packet_id(0xA2)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcCastSpellReq {
    pub spell_slot: SpellSlotBitfield,
    pub position: Vector3,
    pub end_position: Vector3,
    pub target_net_id: u32,
}

#[packet_id(0xA3)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SToggleInputLockingFlag {
    pub input_locking_flags: u32,
}

#[packet_id(0xA5)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SCreateTurret {
    pub unite_net_id: u32,
    pub unit_net_node_id: u8,
    #[serde(with = "crate::string_64")]
    pub name: String,
}

#[packet_id(0xA6)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcDie {
    pub death_data: DeathData,
}

#[packet_id(0xA8)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SShowAuxiliaryText {
    #[serde(with = "crate::string_null")]
    pub text_string_id: String,
}

#[packet_id(0xA9)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SPausePacket {
    pub client_id: u32,
    pub pause_time_remaining: u32,
    pub tournament_pause: bool,
}

#[packet_id(0xAA)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHideObjectiveText;

#[packet_id(0xAB)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnEvent;

#[packet_id(0xAD)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct STeamSurrenderStatus {
    pub reason: u32,
    pub for_vote: u8,
    pub against_vote: u8,
    pub team_id: u32,
}

#[packet_id(0xAF)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHideAuxiliaryText;

#[packet_id(0xB0)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnReplicationAcc {
    pub sync_id: i32,
}

#[packet_id(0xB1)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnDisconnected;

#[packet_id(0xB2)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetGreyscaleEnabledWhenDead {
    pub enabled: bool,
}

#[packet_id(0xB3)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAiState {
    pub ai_state: u32,
}

#[packet_id(0xB4)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetFoWStatus {
    pub enabled: bool,
}

#[packet_id(0xB5)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnEnterLocalVisiblityClient {
    pub max_health: f32,
    pub health: f32,
}

#[packet_id(0xB6)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHighlightShopElement {
    pub element_type: u8,
    pub element_number: u8,
    pub element_sub_category: u8,
}

#[packet_id(0xB8)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SPlayAnimation {
    pub flags: u32,
    pub scale_time: f32,
    #[serde(with = "crate::string_null")]
    pub animation_name: String,
}

#[packet_id(0xB9)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SRefreshAuxiliaryText {
    #[serde(with = "crate::string_null")]
    pub text_string_id: String,
}

#[packet_id(0xBA)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSetFadeOutPush {
    pub fade_id: u16,
    pub fade_time: f32,
    pub fade_target_value: f32,
}
#[packet_id(0xBB)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SOpenTutorialPopup {
    #[serde(with = "crate::string_null")]
    pub message_box_string_id: String,
}

#[packet_id(0xBC)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SRemoveUnitHighlight {
    pub unit_net_id: u32,
}

#[packet_id(0xBD)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SNpcCastSpellAns {
    pub caster_point_sync_id: i32,
    pub cast_info: CastInfo,
}

#[packet_id(0xBF)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNpcBuffAdd {
    pub buff_slot: u8,
    pub buff_type: u8,
    pub count: u8,
    pub is_hidden: bool,
    pub buff_name_hash: u32,
    pub running_time: f32,
    pub duration: f32,
    pub caster_net_id: u32,
}

#[packet_id(0xC1)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SWaypointList {
    pub sync_id: i32,
    pub entries: Vec<Vector2>,
}

#[packet_id(0xC2)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SOnEnterVisibilityClient {
    #[serde(with = "crate::vec_u8")]
    pub entries: Vec<ItemData>,
    // todo
}

#[packet_id(0xC3)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAddDebugCircle {
    pub debug_id: u32,
    pub unit_net_id: u32,
    pub center: Vector3,
    pub radius: f32,
    pub color: Color,
}

#[packet_id(0xC4)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SDisableHUDForEndOfGame;

#[packet_id(0xC7)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SNpcBuffUpdateCountGroup {
    pub duration: f32,
    pub running_time: f32,
    #[serde(with = "crate::vec_u8")]
    pub entries: Vec<BuffUpdateCountGroupEntry>,
}

#[packet_id(0xC8)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAiTargetHero {
    pub target_net_id: u32,
}

#[packet_id(0xC9)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSyncSimTime {
    pub sync_time: f32,
}

#[packet_id(0xC0)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOpenAFKWarningMessage;

#[packet_id(0xCA)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSyncMissionStartTime {
    pub start_time: f32,
}

#[packet_id(0xCB)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SNeutralCampEmpty {
    pub player_net_id: u32,
    pub camp_index: u32,
    pub state: bool,
}

#[packet_id(0xCC)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SOnReplication {
    pub sync_id: i32,
    #[serde(with = "crate::vec_u8")]
    pub replication_data: Vec<ReplicationData>,
}

#[packet_id(0xCD)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SEndOfGameEvent {
    pub team_is_order: bool,
}

#[packet_id(0xCE)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SEndGame {
    pub is_team_order_win: bool,
    pub is_surrender: bool,
}

impl<'de> serde::Deserialize<'de> for SEndGame {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let var: u8 = serde::Deserialize::deserialize(d)?;
        Ok(SEndGame {
            is_team_order_win: var & 0x01 != 0,
            is_surrender: var & 0x02 != 0,
        })
    }
}

impl serde::Serialize for SEndGame {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let byte = ((self.is_team_order_win as u8) << 1) | ((self.is_surrender as u8) << 2);
        s.serialize_u8(byte)
    }
}

#[packet_id(0xD1)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SPopAllCharacterData;

#[packet_id(0xD2)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct STeamSurrenderVote {
    pub bitfield: TeamSurrenderVoteBitfield,
    pub player_net_id: u32,
    pub for_vote: u8,
    pub against_vote: u8,
    pub num_players: u8,
    pub team_id: u32,
    pub time_out: f32,
}

#[packet_id(0xD3)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHandleUiHighlight {
    pub ui_highlight_command: u8,
    pub ui_element: u8,
}

#[packet_id(0xD4)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SFadeMinions {
    pub team_id: u8,
    pub fade_amount: f32,
    pub fade_time: f32,
}

#[packet_id(0xD7)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SShowHealthBar {
    pub show: bool,
}

#[packet_id(0xD8)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SSpawnBot {
    //todo
}

#[packet_id(0xD9)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SSpawnLevelProp {
    pub unit_net_id: u32,
    pub unit_net_node_id: u8,
    pub position: Vector3,
    pub facing: Vector3,
    pub position_offset: Vector3,
    pub team_id: u32,
    pub skill_level: u8,
    pub rank: u8,
    pub typ: u8,
    #[serde(with = "crate::string_64")]
    pub name: String,
    #[serde(with = "crate::string_64")]
    pub prop_name: String,
}

#[packet_id(0xDA)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SUpdateLevelProp {
    pub data: UpdateLevelPropData,
}

#[packet_id(0xDB)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SAttachFlexParticle {
    pub unit_net_id: u32,
    pub flex_id: u8,
    pub cp_index: u8,
    pub attach_type: u8,
}

#[packet_id(0xDC)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHandleCapturePointUpdate {
    pub cp_index: u8,
    pub other_net_id: u32,
    pub par_type: u8,
    pub attack_team: u32,
    pub command: u8,
}

#[packet_id(0xDD)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHandleGameScore {
    pub team_id: u32,
    pub score: i32,
}

#[packet_id(0xDE)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SHandleRespawnPointUpdate {
    pub respawn_point_command: u8,
    pub respawn_point_uiid: u8,
    pub team_id: u32,
    pub client_id: u32,
    pub position: Vector3,
}

#[packet_id(0xE0)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SUnitChangeTeam {
    pub unit_net_id: u32,
    pub team_id: u32,
}

#[packet_id(0xE1)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SUnitSetMinimapIcon {
    pub unit_net_id: u32,
    #[serde(with = "crate::string_64")]
    pub icon_name: String,
}

#[packet_id(0xE2)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SIncrementPlayerScore {
    pub player_net_id: u32,
    pub score_category: u8,
    pub score_event: u8,
    pub is_callout: bool,
    pub point_value: f32,
    pub total_point_value: f32,
}

#[packet_id(0xE3)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SIncrementPlayerStat {
    pub player_net_id: u32,
    pub stat_event: u8,
}

#[packet_id(0xE4)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SColorRemapFX {
    pub is_fading_in: bool,
    pub fade_time: f32,
    pub team_id: u32,
    pub color: Color,
    pub max_weight: f32,
}

#[packet_id(0xE5)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SMusicCueCommand {
    pub music_cue_command: u8,
    pub cue_id: u32,
}

#[packet_id(0xEE)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnEnterTeamVisibility {
    pub visibility_team: u8,
}

#[packet_id(0xEF)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SOnLeaveTeamVisibility {
    pub visibility_team: u8,
}

#[packet_id(0xF0)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SFxOnEnterTeamVisibility {
    pub visibility_team: u8,
}

#[packet_id(0xF1)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SFxOnLeaveTeamVisibility {
    pub visibility_team: u8,
}

#[packet_id(0xF2)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct SReplayOnlyGoldEarned {
    pub owner_net_id: u32,
    pub amount: f32,
}
