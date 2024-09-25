#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![allow(warnings)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;

mod imports;
pub mod spirits;
mod vars;

use spirits::*;
use vars::*;
use crate::imports::imports::*;

#[no_mangle]
pub unsafe extern "C" fn add_battle(battle: spirits::SpiritBattle) {
    let id = battle.battle_id;
    if SPIRIT_BATTLES.try_write().is_none() {
        println!("[spicy_spirits_api] error accessing Spirit Battles Map");
        return;
    }
    let mut battlemanager = SPIRIT_BATTLES.write();
    battlemanager.battles.push(battle);
    println!("[spicy_spirits_api] added battle {id}");
}

#[no_mangle]
pub unsafe extern "C" fn is_ready() -> bool {
    return is_valid_battle() && sv_information::is_ready_go();
}
#[no_mangle]
pub unsafe extern "C" fn is_ready_init() -> bool {
    let to_return = *IS_READY_INIT.read();
    if to_return == true {
        //set_ready_init(false);
    }
    return to_return;
}
#[no_mangle]
pub unsafe extern "C" fn set_ready_init(state: bool) {
    *IS_READY_INIT.write() = state;
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_battle() -> bool {
    return is_valid_map() && get_sprit_battle_id() > 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_map() -> bool {
    return *IS_VALID_MAP.read();
}
#[no_mangle]
pub unsafe extern "C" fn set_valid_map(stage_id: i32) {
    let is_valid = (!*StageID::Training..*StageID::Staffroll).contains(&(stage_id));
    return *IS_VALID_MAP.write() = is_valid;
}

#[no_mangle]
pub unsafe extern "C" fn get_sprit_battle_id() -> u64 {
    return *CURRENT_BATTLE_ID.read();
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_sprit_battle_id(id: u64) {
    return *CURRENT_BATTLE_ID.write() = id;
}
#[no_mangle]
pub unsafe extern "C" fn set_sprit_battle_id_from_battle(compare_against: &mut SpiritBattle) {
    unsafe {
        println!("[spicy_spirits_api] Setting id...");
        let mut battlemanager = SPIRIT_BATTLES.read();
        for battle in (&battlemanager.battles) {
            if (*battle == *compare_against) {
                let battle_id = (*battle).battle_id;
                (*compare_against).battle_id = battle_id; //technically not necessary but w/e
                set_sprit_battle_id(battle_id);
                if (battle_id>0) {
                    println!("[spicy_spirits_api] Current Battle: {battle_id}");
                }
                break;
            }
        }
    }
}