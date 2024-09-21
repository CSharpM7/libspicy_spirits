use crate::imports::imports::*;
use once_cell::sync::Lazy;
use std::{
    sync::Mutex
};
use crate::spirits::*;


pub static mut IS_READY: bool = false;
pub static mut IS_LOADED: bool = false;
pub static mut IN_INVALID_MAP: bool = false;
pub static mut FIGHT_STAGE_ID: i32 = -1;


use parking_lot::RwLock;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref SPIRIT_BATTLES: RwLock<SpiritBattleManager> = RwLock::new(SpiritBattleManager::new());
    pub static ref CURRENT_BATTLE: RwLock<SpiritBattle> = RwLock::new(SpiritBattle::new());
    pub static ref CURRENT_BATTLE_ID: RwLock<u64> = RwLock::new(0);
    pub static ref IS_VALID_MAP: RwLock<bool> = RwLock::new(false);
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