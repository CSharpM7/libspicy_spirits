use crate::imports::imports_agent::*;
use once_cell::sync::Lazy;
use std::{
    sync::Mutex
}; 


pub static mut IS_READY: bool = false;
pub static mut IS_LOADED: bool = false;
pub static mut IN_INVALID_MAP: bool = false;
pub static mut FIGHT_STAGE_ID: i32 = -1;
