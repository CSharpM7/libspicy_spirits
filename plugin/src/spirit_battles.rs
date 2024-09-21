/*
Use this file to add various spirit battles to be checked. Make sure the enemies are only the ones that are initially spawned in
*/
use crate::imports::imports_agent::*;
use spicy_spirits::spirits::*;

fn load_battles() {
    unsafe {
        let prog_enemies: Vec<SpiritEnemy> = vec![SpiritEnemy{kind: *FIGHTER_KIND_MARIO,color: 3}];
        let prog = SpiritBattle {
            battle_id: hash40("smoky_progg"),
            battle_type: RULESET_STOCK,
            basic_init_hp: 0.0,
            basic_stock: 1,
            stage_id: *StageID::Battle_Pikmin_Planet,
            enemies: prog_enemies,
        };
        println!("[spicy_spirits_mod] Plugin: adding smoky prog...");
        spicy_spirits::add_battle(prog);
    }
}

pub fn install() {
    load_battles();
}