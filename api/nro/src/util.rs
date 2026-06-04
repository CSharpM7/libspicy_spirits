use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{self,lua_bind::*, *},
        lib::{lua_const::*, L2CValue},
        hash40,
        phx::*
    },
    smash_script::{
        *,
        macros::*
    }
};

#[skyline::from_offset(0x3ac560)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;
extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}

pub fn get_active_battle_object_id_from_entry_id(entry_id: u32) -> Option<u32> {
    use smash::lib::lua_const::*;
    use smash::app::lua_bind::*;
    let object = get_battle_object_from_entry_id(entry_id)?;
    if object.is_null() { return None; }
    let object = unsafe { &mut *object };
    let kind = object.kind as i32;
    let status = unsafe {
        StatusModule::status_kind(object.module_accessor)
    };
    if status != *FIGHTER_STATUS_KIND_NONE && status != *FIGHTER_STATUS_KIND_STANDBY {
        return Some(object.battle_object_id);
    }
    if kind == *FIGHTER_KIND_ELIGHT || kind == *FIGHTER_KIND_EFLAME {
        Some(object.battle_object_id + 0x10000)
    } else if kind == *FIGHTER_KIND_PZENIGAME || kind == *FIGHTER_KIND_PFUSHIGISOU || kind == *FIGHTER_KIND_PLIZARDON {
        let next_id = object.battle_object_id + 0x10000;
        let next_object = unsafe { &mut *get_battle_object_from_id(next_id) };
        let next_status = unsafe {
            StatusModule::status_kind(next_object.module_accessor)
        };
        if next_status != *FIGHTER_STATUS_KIND_NONE && next_status != *FIGHTER_STATUS_KIND_STANDBY {
            Some(next_id)
        } else {
            Some(next_id + 0x10000)
        }
    } else {
        Some(object.battle_object_id)
    }
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}

```rust
#[repr(u8)]
#[derive(Debug)]
enum BattleMode {
    Time = 0,
    Stock = 1,
    Stamina = 2,
}

#[repr(u8)]
#[derive(Debug)]
enum ItemAppear {
    None = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Highest = 4,
}

#[repr(u8)]
#[derive(Debug)]
enum MeleeMode {
    Normal=0,
    NormalMulti=1,
    TeamCompe=2,
    TeamCompeMulti=3,
    Standard=4,
    StandardMulti=5,
    AllBattle=6,
    SpiritsBattle=7,
    SpiritsBattleMulti=8,
    ContestHomerun=9,
    ContestKumite=10,
    ContestKumiteMulti=11,
    TrainingSandbag=12,
    ShamBattle=13,
    AnyoneSingle=14,
    AnyoneDouble=15,
    AnyoneVIP=16,
    AnyoneAmiibo=17,
    AnyoneSpirits=18,
    FriendSpirits=19,
    AnyoneRoom=20,
    FriendRoom=21,
    AnyoneCoop=22,
    FriendCoop=23,
    OnlineTournament=24,
    OnlineWatching=25,
    OnlineConquest=26,
    CollectionStageEdit=27,
    MenuFighter=28,
    Unknown1=29,
    GameplayDemo=30,
    StaffRoll=31,
    SingleTrainingOffline=32,
    SingleHowTo=33,
    Unknown2=34,
    VR=35,
    VRMulti=36,
    OnlineConvention=37
}

#[repr(C)]
#[derive(Debug)]
struct SelectedRuleset {
    pub unk: [u8; 0x30],
    pub melee_mode1: MeleeMode,
    pub melee_mode2: MeleeMode,
    pub battle_mode: BattleMode,
    pub unk1: [u8; 0x6],
    pub is_super_sudden_death: bool,
    pub is_sudden_death: bool,
    pub unk2: [u8; 0x6],
    pub is_melee_large: bool,
    pub unk3: [u8; 0x2],
    pub is_team_attack: bool,
    pub unk4: [u8; 0x13],
    pub is_spirits_battle: bool,
    pub unk5: [u8; 0xF],
    pub is_team_battle: bool,
    pub unk6: [u8; 0xF],
    pub timer: u32, // timer is in frames. Set to 0 for inf time.
    pub stock_count: u32,
    pub unk7: [u8; 0x4],
    pub item_appear: ItemAppear,
    pub unk8: [u8; 0x3c3], // Item enabled/disabled most likely in here
    pub stage_morph_timer: u32, // 0xFF_FF_FF_FF -> Random, 0x0 -> Off, Frame Count -> Time
    pub unk9: [u8; 0x4],
}

const SELECTED_RULESET_OFFSET: usize = 0x52c4180; // 13.0.4
```