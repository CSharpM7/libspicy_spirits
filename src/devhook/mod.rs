/*
GOAL:
Have the normal plugin create Spirit Parameters to be loaded into here on boot (or move this into here).

When the normal plugin needs to reset, it should only reset the fighter frame stuff. This plugin should export a function
for checking if the current match could be a spirit, and returning the spirit id
*/

use crate::imports::imports_agent::*;
use once_cell::sync::Lazy;
use std::{
    sync::Mutex
};
use crate::vars::*;




use parking_lot::RwLock;
use lazy_static::lazy_static;
//pub static SPIRIT_BATTLES: Lazy<Mutex<Vec<SpiritBattle>>> = Lazy::new(|| {Mutex::new(Vec::new())});
lazy_static! {
    pub static ref SPIRIT_BATTLES: RwLock<SpiritBattleManager> = RwLock::new(SpiritBattleManager::new());
    pub static ref CURRENT_BATTLE: RwLock<SpiritBattle> = RwLock::new(SpiritBattle::new());
    pub static ref CURRENT_BATTLE_ID: RwLock<u64> = RwLock::new(0);
    pub static ref IS_INVALID_MAP: RwLock<bool> = RwLock::new(false);
}
pub struct SpiritBattleManager {
    pub battles: Vec<SpiritBattle>
}
impl SpiritBattleManager {
    pub(crate) fn new() -> Self {
        Self {
            battles: Vec::new(),
        }
    }
}

fn load_battles() {
    unsafe {
        let mut battlemanager = SPIRIT_BATTLES.write();
        let prog_enemies: Vec<SpiritEnemy> = vec![SpiritEnemy{kind: *FIGHTER_KIND_MARIO,color: 3}];
        let prog = SpiritBattle {
            battle_id: hash40("smoky_progg"),
            battle_type: RULESET_STOCK,
            basic_init_hp: 0.0,
            basic_stock: 1,
            stage_id: *StageID::Battle_Pikmin_Planet,
            enemies: prog_enemies,
        };
        battlemanager.battles.push(prog);
    }
}
/* 
unsafe fn find_battle(compare_against: &SpiritBattle) {
    let mut battlemanager = SPIRIT_BATTLES.read();
    for battle in (&battlemanager.battles) {
        //if (battle.battle_type)
        if (*battle == *compare_against) {
            println!("WOAH");
        }
    }
}
*/
#[no_mangle]
pub unsafe extern "Rust" fn is_invalid_map() -> bool {
    return *IS_INVALID_MAP.read();
}
#[no_mangle]
pub unsafe extern "Rust" fn get_sprit_battle_id() -> u64 {
    return *CURRENT_BATTLE_ID.read();
}
fn set_battle_id(compare_against: &mut SpiritBattle) -> bool {
    unsafe {
        let mut battlemanager = SPIRIT_BATTLES.read();
        for battle in (&battlemanager.battles) {
            //if (battle.battle_type)
            if (*battle == *compare_against) {
                let battle_id = (*battle).battle_id;
                (*compare_against).battle_id = battle_id; //technically not necessary but w/e
                *CURRENT_BATTLE_ID.write() = battle_id;
                println!("Current Battle: {battle_id}");
                break;
            }
        }
    }
    false
}

unsafe fn startup_load_battle(fighter: &mut L2CFighterCommon) {
    let entries = app::lua_bind::FighterManager::entry_count(singletons::FighterManager()) as u32;

    //Find out about the ruleset
    let player_info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(0));
    let stocks = app::lua_bind::FighterInformation::stock_count(player_info);
    let hp = app::lua_bind::FighterInformation::hit_point_max(player_info, false);
    //println!("Ruleset: Stocks {stocks} HP {hp}");
    let ruleset = if stocks == 0 {RULESET_TIME}
    else if hp == 0.0 {RULESET_STOCK}
    else {RULESET_HP};

    let mut enemies: Vec<SpiritEnemy> = vec![];
    for entry_id in 1..entries {
        let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
        //let is_cpu = app::lua_bind::FighterInformation::is_operation_cpu(info); 
        //let stocks = app::lua_bind::FighterInformation::stock_count(info);
        let enemy_hp = app::lua_bind::FighterInformation::hit_point_max(info, false);
        let enemy_color = app::lua_bind::FighterInformation::fighter_color(info);
        let enemy_id = get_active_battle_object_id_from_entry_id(entry_id).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        if enemy_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            let enemy_obj = get_battle_object_from_id(enemy_id);
            let enemy_boma = &mut *(*enemy_obj).module_accessor;
            let enemy_kind = smash::app::utility::get_kind(enemy_boma);
            println!("Entry {entry_id}: Kind:{enemy_kind} (c0{enemy_color})");
            let enemy = SpiritEnemy{
                kind: enemy_kind,
                color: enemy_color,
            };
            enemies.push(enemy);
        }
        else {
            println!("Entry {entry_id} inactive");
        }
    }

    println!("Ruleset: {ruleset} ({hp}%*{stocks}) on {FIGHT_STAGE_ID}");
    let mut fight = SpiritBattle {
        battle_id: 0,
        battle_type: ruleset,
        basic_init_hp: hp,
        basic_stock: stocks,
        stage_id: FIGHT_STAGE_ID,
        enemies: enemies,
    };
    let progg =  &SPIRIT_BATTLES.read().battles[0];

    let p_ruleset = progg.battle_type;
    let p_hp = progg.basic_init_hp;
    let p_stocks = progg.basic_stock;
    let p_stage_id = progg.stage_id;
    println!("Progg: {p_ruleset} ({p_hp}%*{p_stocks}) on {p_stage_id}");

    set_battle_id(&mut fight);
    //let mut current_battle = CURRENT_BATTLE.write();
    //current_battle.write() = fight;
}

unsafe fn startup_set_ready(fighter: &mut L2CFighterCommon) {
    let entry_id =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if entry_id == 0 {
        if sv_information::is_ready_go() && !IS_READY {
            println!("READY");
            IS_READY = true;
        }
        else if !sv_information::is_ready_go() && !IS_LOADED {
            if fighter.global_table[STATUS_FRAME].get_f32() >= 15.0 {
                println!("LOAD SETTINGS");
                IS_LOADED = true;
                startup_load_battle(fighter);
            }
        }
    }
}

unsafe fn startup_set_map(fighter: &mut L2CFighterCommon) {
    *CURRENT_BATTLE_ID.write() = 0;
    let entry_id = sv_battle_object::entry_id(fighter.battle_object_id);
    let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if kind != *FIGHTER_KIND_NANA {
        //Init Settings
        if entry_id == 0 {   
            IS_READY = false;
            IS_LOADED = false;
            let stage_id = stage::get_stage_id();
            FIGHT_STAGE_ID = stage_id;
            let invalid = (*StageID::Training..*StageID::Staffroll).contains(&(stage_id as i32));
            IN_INVALID_MAP = invalid;
            *IS_INVALID_MAP.write() = invalid;
            *CURRENT_BATTLE_ID.write() = 0;
            println!("Stage: {stage_id} ({IN_INVALID_MAP})");
        }
    }
}

unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    if !IN_INVALID_MAP {
        startup_set_ready(fighter);
    }
}

pub unsafe extern "C" fn fighter_start(fighter: &mut L2CFighterCommon)
{
    startup_set_map(fighter);
}

pub fn install() {
    load_battles();
    smashline::Agent::new("fighter")
        .on_line(Main,fighter_frame)
        .on_start(fighter_start)
        .install();
}