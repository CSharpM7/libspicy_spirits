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

//#[macro_use]
//extern crate lazy_static;

mod imports;
pub mod spirits;
pub mod util;
mod system;
mod vars;


#[skyline::main(name = "libspicy_spirits")]
pub fn main() {
    //system::install();
    println!("HUH");
}


use spirits::*;
use vars::*;
#[no_mangle]
pub unsafe extern "C" fn add_battle(battle: spirits::SpiritBattle) {
    let mut battlemanager = SPIRIT_BATTLES.write();
    battlemanager.battles.push(battle);
}

#[no_mangle]
pub unsafe extern "C" fn is_invalid_map() -> bool {
    return *IS_INVALID_MAP.read();
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn get_sprit_battle_id() -> u64 {
    return *CURRENT_BATTLE_ID.read();
    return 0;
}