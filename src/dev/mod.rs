use crate::imports::imports_agent::*;
use once_cell::sync::Lazy;
use std::{
    sync::Mutex
};
use crate::vars::*;

/* 
struct SpiritEnemy {
    kind: i32,
    color: u64,
}
impl PartialEq for SpiritEnemy {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind &&
        self.color == other.color
    }
}
fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

struct SpiritBattle {
    battle_type: i32,
    basic_init_hp: f32,
    basic_stock: u64,
    stage_id: i32,
    enemies: Vec<SpiritEnemy>
}
impl PartialEq for SpiritBattle {
    fn eq(&self, other: &Self) -> bool {
        self.battle_type == other.battle_type &&
        self.basic_init_hp == other.basic_init_hp &&
        self.basic_stock == other.basic_stock &&
        self.stage_id == other.stage_id &&
        do_vecs_match(&self.enemies,&other.enemies)
    }
}


use parking_lot::RwLock;
use lazy_static::lazy_static;
//pub static SPIRIT_BATTLES: Lazy<Mutex<Vec<SpiritBattle>>> = Lazy::new(|| {Mutex::new(Vec::new())});
lazy_static! {
    pub static ref SPIRIT_BATTLES: RwLock<SpiritBattleManager> = RwLock::new(SpiritBattleManager::new());
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
            battle_type: RULESET_STOCK,
            basic_init_hp: 0.0,
            basic_stock: 1,
            stage_id: *StageID::Battle_Pikmin_Planet,
            enemies: prog_enemies,
        };
        battlemanager.battles.push(prog);
    }
}
unsafe fn find_battle(compare_against: &SpiritBattle) {
    let mut battlemanager = SPIRIT_BATTLES.read();
    for battle in (&battlemanager.battles) {
        //if (battle.battle_type)
        if (*battle == *compare_against) {
            println!("WOAH");
        }
    }
}

unsafe fn load_settings() {
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
    let fight = SpiritBattle {
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

    find_battle(&fight);
}

*/

extern "Rust" {
    #[link_name = "is_invalid_map"]
    pub fn is_invalid_map()->bool;

    #[link_name = "get_sprit_battle_id"]
    pub fn get_sprit_battle_id()->u64;
}
/*
pub unsafe fn get_sprit_battle_id() -> u64 {
    return hash40("smoky_progg");
} 
*/
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let entry_id =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if entry_id == 0 {
        if get_sprit_battle_id() != 0 {
            if sv_information::is_ready_go() {
                if !IS_READY {
                    IS_READY = true;
                    let is_progg = get_sprit_battle_id() == hash40("smoky_progg");
                    println!("READY: {is_progg}");
                }
            }
        }
    }
}

pub unsafe extern "C" fn fighter_start(fighter: &mut L2CFighterCommon)
{
    let entry_id = sv_battle_object::entry_id(fighter.battle_object_id);
    let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if kind != *FIGHTER_KIND_NANA 
    && entry_id == 0 {
        println!("Fighter Start");
        //RESET VARIABLES
        IN_INVALID_MAP = is_invalid_map();
        IS_READY = false;
    }
}
pub fn install() {
    smashline::Agent::new("fighter")
        .on_line(Main,fighter_frame)
        //.on_start(fighter_start)
        .install();
}