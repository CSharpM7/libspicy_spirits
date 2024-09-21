#![feature(proc_macro_hygiene)]

use skyline::{hook, install_hook};
use crate::imports::imports_agent::*;
use smash::lib::lua_const::StageID::*;

extern "C" {
    #[link_name = "_ZN3app5stage12get_stage_idEv"]
    fn get_stage_id() -> u32;
    fn get_current_stage_alt() -> usize;
}


#[skyline::hook(offset = 0x178ab60, inline)]
unsafe fn init_stage(ctx: &mut skyline::hooks::InlineCtx) {
    let stage_id = *ctx.registers[1].w.as_ref();
    let is_alt_haz_off = ([0x59].contains(&stage_id) && get_current_stage_alt() == 0)
        || (stage_id == 0x68 && get_current_stage_alt() == 0);
    let fighters = app::lua_bind::FighterManager::entry_count(singletons::FighterManager());
    IS_READY = false;
    /*
           Training: 0x134,
        SettingStage: 0x135,
        ResultStage: 0x136,
        ShamFight: 0x137,
        SpiritsRoulette: 0x138,
        CampaignMap: 0x139,
        BonusGame: 0x13A,
        HomerunContest: 0x13B,
        Staffroll: 0x13C,
         */
    //IN_INVALID_MAP = (*StageID::Training..*StageID::Staffroll).contains(&(stage_id as i32));
    //println!("Load stage: {stage_id} Valid: {IN_INVALID_MAP}");
}


#[skyline::hook( replace = StatusModule::init_settings )]
pub unsafe fn init_settings_replace(
    module_accessor: *mut BattleObjectModuleAccessor,
    situation: SituationKind,
    kinetic: i32,
    correct: u32,
    cliff_check: GroundCliffCheckKind,
    jostle: bool,
    keep_flag: i32,
    keep_int: i32,
    keep_float: i32,
    arg10: i32,
) {
    if !IS_READY && !IN_INVALID_MAP {
        //IN_INVALID_MAP
    }
    original!()(
        module_accessor,
        situation,
        kinetic,
        correct,
        cliff_check,
        jostle,
        keep_flag,
        keep_int,
        keep_float,
        arg10
    )
}

#[skyline::hook(offset = 0x3afa10)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    let entry_id =  WorkModule::get_int((module_accessor), *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    println!("[CustomVarManager] Start {entry_id}");
}

pub fn install() {
    skyline::install_hooks!(
        init_stage,
        //init_settings_replace,
        //battleobjectmoduleaccessor__start_modules
    );
}