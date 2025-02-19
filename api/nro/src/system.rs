use crate::imports::imports_agent::*;
use spicy_spirits::{*,spirits::*};
use crate::util::*;

pub static mut IS_FIRST_FIGHTER_LOADED: bool = false;
pub static mut IS_READY: bool = false;
pub static mut IS_LOADED: bool = false;
pub static mut IS_END: bool = false;
pub const OFFSET_ONCE_PER_FRAME: usize = 0x135b810;
static mut IS_PENDING_NEW_GAME: bool = false;
static mut PREVIOUS_GAME_STATE_PTR: u64 = 0;

unsafe fn startup_set_info(fighter: &mut L2CFighterCommon) {
    let entries = app::lua_bind::FighterManager::entry_count(singletons::FighterManager()) as u32;
    let stage = stage::get_stage_id();

    //Find out about the ruleset
    let player_info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(0));
    let enemy_info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(1));
    let stocks = app::lua_bind::FighterInformation::stock_count(player_info);
    let hp = app::lua_bind::FighterInformation::hit_point_max(enemy_info, false);
    let ruleset = if stocks == 0 {RULESET_TIME}
    else if hp == 0.0 {RULESET_STOCK}
    else {RULESET_STAMINA};

    //Find out about enemies
    let mut enemies: Vec<SpiritEnemy> = vec![];
    for entry_id in 1..entries {
        let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
        //let is_cpu = app::lua_bind::FighterInformation::is_operation_cpu(info); 
        //let stocks = app::lua_bind::FighterInformation::stock_count(info);
        //let enemy_hp = app::lua_bind::FighterInformation::hit_point_max(info, false);
        let enemy_color = app::lua_bind::FighterInformation::fighter_color(info) as i32;
        let enemy_id = get_active_battle_object_id_from_entry_id(entry_id).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        if enemy_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            let enemy_obj = get_battle_object_from_id(enemy_id);
            let enemy_boma = &mut *(*enemy_obj).module_accessor;
            let enemy_kind = smash::app::utility::get_kind(enemy_boma);
            if enemy_kind < 0 {
                continue;
            }
            println!("[spicy_spirits_nro] Entry {entry_id}: Kind:{enemy_kind} (c0{enemy_color})");
            let enemy = SpiritEnemy{
                kind: enemy_kind,
                color: enemy_color,
            };
            enemies.push(enemy);
        }
        else {
            println!("[spicy_spirits_nro] Entry {entry_id} inactive");
        }
    }

    println!("[spicy_spirits_nro] Ruleset: {ruleset} ({hp}%*{stocks}) on {stage}");
    let mut fight = SpiritBattle {
        battle_id: 0,
        battle_type: ruleset,
        basic_init_hp: hp,
        basic_stock: stocks,
        stage_id: stage,
        enemies: enemies,
    };
    spicy_spirits::set_sprit_battle_id_from_battle(&mut fight);
}

unsafe fn startup_set_ready(fighter: &mut L2CFighterCommon) {
    let entry_id =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if entry_id == 0 {
        //During battle
        if sv_information::is_ready_go() 
        && spicy_spirits::get_sprit_battle_id() > 0 {
            if !IS_READY {
                println!("[spicy_spirits_nro] READY");
                IS_READY = true;
                spicy_spirits::set_ready_init(true);
            }
            else {
                spicy_spirits::set_ready_init(false);
            }
        }
        //Before Battle
        else if !sv_information::is_ready_go() && !IS_LOADED {
            spicy_spirits::set_end_init(false);
            IS_END = false;
            if fighter.global_table[0xE].get_f32() >= 15.0 {
                println!("[spicy_spirits_nro] Set Battle Info");
                IS_LOADED = true;
                IS_FIRST_FIGHTER_LOADED = false;
                startup_set_info(fighter);
            }
        }
        //After Battle
        else if !sv_information::is_ready_go() && IS_LOADED && IS_READY {
            if !IS_END {
                println!("[spicy_spirits_nro] END");
                IS_END = true;
                spicy_spirits::set_end_init(true);
            }
            else {
                spicy_spirits::set_end_init(false);
            }
        }
    }
}

unsafe fn startup_set_map(fighter: &mut L2CFighterCommon) {
    let entry_id = sv_battle_object::entry_id(fighter.battle_object_id);
    let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if kind != *FIGHTER_KIND_NANA {
        //Init Settings
        if entry_id == 0 {  
            spicy_spirits::set_sprit_battle_id(0); 
            spicy_spirits::set_ready_init(false);
            IS_FIRST_FIGHTER_LOADED = true;
            IS_READY = false;
            IS_LOADED = false;
            let stage_id = stage::get_stage_id();
            spicy_spirits::set_valid_map(stage_id);
            if spicy_spirits::is_valid_game_mode() {
                println!("[spicy_spirits_nro] Stage: {stage_id}");
            }
            else {
                println!("[spicy_spirits_nro] Currently not playing Adventure/Spirit Board!");
            }
        }
    }
}

unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    if spicy_spirits::is_valid_map() && spicy_spirits::is_valid_game_mode() {
        startup_set_ready(fighter);
    }
}

pub unsafe extern "C" fn fighter_start(fighter: &mut L2CFighterCommon)
{
    startup_set_map(fighter);
}
/*
TODO: use once per game frame instead of opff to set up plugin.
Need to figure out how to check if fighters are loaded in.


#[skyline::hook(offset = OFFSET_ONCE_PER_FRAME)]
unsafe fn once_per_game_frame(game_state_ptr: u64) {

    // check the current match mode
    // 1 is regular smash, 45 is online arena match
    let match_mode = get_match_mode().0;
    let valid_mode = [
        7, //Spirit Board
        8, //Adventure
        18, //Training
        63 //DLC Spirit Board
    ].contains(&match_mode);
    //println!("mode is {}", match_mode);
    if match_mode != 1 && match_mode != 45 {
    }
    if PREVIOUS_GAME_STATE_PTR != game_state_ptr {
        //New Mode
        spicy_spirits::set_sprit_battle_id(0); 
        spicy_spirits::set_ready_init(false);
        IS_READY = false;
        IS_LOADED = false;
        let stage_id = stage::get_stage_id();
        spicy_spirits::set_valid_map(stage_id);
        println!("[spicy_spirits_nro] Stage: {stage_id}");
    }
    PREVIOUS_GAME_STATE_PTR = game_state_ptr;
    let entries = app::lua_bind::FighterManager::entry_count(singletons::FighterManager()) as u32;
    if valid_mode 
    && (IS_FIRST_FIGHTER_LOADED || IS_LOADED) {
        //During battle
        if sv_information::is_ready_go() 
        && spicy_spirits::get_sprit_battle_id() > 0 {
            if !IS_READY {
                println!("[spicy_spirits_nro] READY");
                IS_READY = true;
                spicy_spirits::set_ready_init(true);
            }
            else {
                spicy_spirits::set_ready_init(false);
            }
        }
        //Before Battle
        else if !sv_information::is_ready_go() && !IS_LOADED {
            spicy_spirits::set_end_init(false);
            IS_END = false;
            if (entries > 0) {
                println!("Fighters loaded");
            }
            /* 
            if fighter.global_table[0xE].get_f32() >= 15.0 {
                println!("[spicy_spirits_nro] Set Battle Info");
                IS_LOADED = true;
                startup_set_info(fighter);
            }
            */
        }
        //After Battle
        else if !sv_information::is_ready_go() && IS_LOADED && IS_READY {
            if !IS_END {
                println!("[spicy_spirits_nro] END");
                IS_END = true;
                spicy_spirits::set_end_init(true);
            }
            else {
                spicy_spirits::set_end_init(false);
            }
        }
    }

    call_original!(game_state_ptr)
}
*/
pub fn install() {
    smashline::Agent::new("fighter")
        .on_line(Main,fighter_frame)
        .on_start(fighter_start)
        .install(); 
    //skyline::install_hooks!(once_per_game_frame);
}