use crate::imports::imports_agent::*;
use spicy_spirits::{*,spirits::*};
use crate::vars::*;
use crate::util::*;

unsafe fn startup_set_info(fighter: &mut L2CFighterCommon) {
    let entries = app::lua_bind::FighterManager::entry_count(singletons::FighterManager()) as u32;
    let stage = stage::get_stage_id();

    //Find out about the ruleset
    let player_info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(0));
    let stocks = app::lua_bind::FighterInformation::stock_count(player_info);
    let hp = app::lua_bind::FighterInformation::hit_point_max(player_info, false);
    let ruleset = if stocks == 0 {RULESET_TIME}
    else if hp == 0.0 {RULESET_STOCK}
    else {RULESET_HP};

    //Find out about enemies
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
        else if !sv_information::is_ready_go() && !IS_LOADED {
            if fighter.global_table[0xE].get_f32() >= 15.0 {
                println!("[spicy_spirits_nro] Set Battle Info");
                IS_LOADED = true;
                startup_set_info(fighter);
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
            IS_READY = false;
            IS_LOADED = false;
            let stage_id = stage::get_stage_id();
            spicy_spirits::set_valid_map(stage_id);
            println!("[spicy_spirits_nro] Stage: {stage_id} ({IN_INVALID_MAP})");
        }
    }
}

unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    if spicy_spirits::is_valid_map() {
        startup_set_ready(fighter);
    }
}

pub unsafe extern "C" fn fighter_start(fighter: &mut L2CFighterCommon)
{
    startup_set_map(fighter);
}
pub fn install() {
    smashline::Agent::new("fighter")
        .on_line(Main,fighter_frame)
        .on_start(fighter_start)
        .install();
}