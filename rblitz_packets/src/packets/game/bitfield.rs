make_bitfield! {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TeamSurrenderVoteBitfield = bitfield: u8 {
        vote_yes: bool = bitfield & 1 != 0,
        open_vote_menu: bool = bitfield & 2 != 0,
    }
}

impl serde::Serialize for TeamSurrenderVoteBitfield {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut byte = 0;
        if self.vote_yes {
            byte |= 1;
        }
        if self.open_vote_menu {
            byte |= 2;
        }
        s.serialize_u8(byte)
    }
}

make_bitfield! {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct MapPingBitfield = bitfield: u8 {
        ping_category: u8 = bitfield & 0x0F,
        play_audio: bool = (bitfield & 0x10) != 0,
        show_chat: bool = (bitfield & 0x20) != 0,
        ping_throttled: bool = (bitfield & 0x40) != 0,
    }
}

impl serde::Serialize for MapPingBitfield {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let byte = self.ping_category
            | ((self.play_audio as u8) << 0x10)
            | ((self.show_chat as u8) << 0x20)
            | ((self.ping_throttled as u8) << 0x40);
        s.serialize_u8(byte)
    }
}

make_bitfield! {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SpawnMinionBitfield = bitfield: u8 {
        ignore_collision: bool = (bitfield & 1) != 0,
        is_ward: bool = (bitfield & 2) != 0,
        use_behaviour_tree_ai: bool = (bitfield & 4) != 0,
    }
}

impl serde::Serialize for SpawnMinionBitfield {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut byte = 0;
        if self.ignore_collision {
            byte |= 0x01;
        }
        if self.is_ward {
            byte |= 0x02;
        }
        if self.use_behaviour_tree_ai {
            byte |= 0x04;
        }
        s.serialize_u8(byte)
    }
}

make_bitfield! {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CastInfoBitfield = bitfield: u8 {
        is_auto_attack: bool = (bitfield & 1) != 0,
        is_second_auto_attack: bool = (bitfield & 2) != 0,
        is_force_casting_or_channel: bool = (bitfield & 4) != 0,
        is_override_cast_position: bool = (bitfield & 8) != 0,
    }
}

impl serde::Serialize for CastInfoBitfield {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut byte = 0;
        if self.is_auto_attack {
            byte |= 0x01;
        }
        if self.is_second_auto_attack {
            byte |= 0x02;
        }
        if self.is_force_casting_or_channel {
            byte |= 0x04;
        }
        if self.is_override_cast_position {
            byte |= 0x08;
        }
        s.serialize_u8(byte)
    }
}

make_bitfield! {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SpellSlotBitfield = bitfield: u8 {
        slot: u8 = bitfield & ((1 << 7) - 1),//bit 1 to 7, 0x7F
        is_summoner_spell: bool = bitfield & (1 << 7) != 0, //bit 8 0x80
    }
}

impl serde::Serialize for SpellSlotBitfield {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut byte = self.slot & 0x7F;
        if self.is_summoner_spell {
            byte |= 0x80;
        }
        s.serialize_u8(byte)
    }
}

make_bitfield! {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct ShieldProperties = bitfield: u8 {
        phyiscal: bool = bitfield & 1 != 0,
        magical: bool = bitfield & 2 != 0,
        stop_shield_fade: bool = bitfield & 4 != 0,
    }
}

impl serde::Serialize for ShieldProperties {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut byte = 0;
        if self.phyiscal {
            byte |= 1;
        }
        if self.magical {
            byte |= 2;
        }
        if self.stop_shield_fade {
            byte |= 4;
        }
        s.serialize_u8(byte)
    }
}

make_bitfield! {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct CharSpawnPetBitfield = bitfield: u8 {
        copy_inventory: bool = bitfield & 1 != 0,
        clear_focus_target: bool = bitfield & 2 != 0,
    }
}

impl serde::Serialize for CharSpawnPetBitfield {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut byte = 0;
        if self.copy_inventory {
            byte |= 1;
        }
        if self.clear_focus_target {
            byte |= 2;
        }
        s.serialize_u8(byte)
    }
}
