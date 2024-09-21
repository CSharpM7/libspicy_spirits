/*
Use this file to check if the current battle is a modded spirit battle, if so then do some spicy stuff
*/
use crate::imports::imports_agent::*;

unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let entry_id =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
    let is_cpu = smash::app::lua_bind::FighterInformation::is_operation_cpu(info);
    let spirit_battle = spicy_spirits::get_sprit_battle_id();
    if spicy_spirits::is_ready() {
        if spirit_battle == hash40("smoky_progg") {
            if entry_id == 0 {
                crate::spirit_scripts::smoky_progg::player_frame(fighter);
                if spicy_spirits::is_ready_init() {
                    crate::spirit_scripts::smoky_progg::player_init(fighter);
                }
            }
        }
    }
}

pub fn install() {
    smashline::Agent::new("fighter")
        .on_line(Main,fighter_frame)
        //.on_start(fighter_start)
        .install();
}