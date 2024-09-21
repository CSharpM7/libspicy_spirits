use crate::imports::imports_agent::*;

pub static mut ENEMY_EFFECT_ID: u32 = 0;
pub static mut ENEMY_OBJECT_ID: u32 = 0;
pub static mut FIGHTER_POISON_FRAME: i32 = 0;
pub const FIGHTER_POISON_FRAME_MAX: i32 = 7;
pub const FIGHTER_POISON_DIST_MAX: f32 = 35.0;
pub const POISON_FX_DIST_MAX: f32 = 6.0;


unsafe fn player_frame(fighter: &mut L2CFighterCommon) {
    if ENEMY_OBJECT_ID > 0 && sv_battle_object::is_active(ENEMY_OBJECT_ID) {
        let scale = 1.0;
        let enemy_boma = sv_battle_object::module_accessor(ENEMY_OBJECT_ID);
        
        let pos = *PostureModule::pos(fighter.module_accessor);
        let other_pos = *PostureModule::pos(enemy_boma);
        //let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
        //let other_height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
        let mut offset = Vector3f::zero();
        let mut other_offset = Vector3f::zero();
        ModelModule::joint_global_offset_from_top(fighter.module_accessor, Hash40{hash: hash40("rot")}, &mut offset); 
        //offset = Vector3f{x: 0.0, y: 3.0*scale, z: 0.0 };
        ModelModule::joint_global_offset_from_top(enemy_boma, Hash40{hash: hash40("rot")}, &mut other_offset);  
        
        let x = pos.x;
        let other_x = other_pos.x;
        let y = pos.y+offset.y;
        let other_y = other_pos.y+other_offset.y;
        let dist = sv_math::vec2_distance(pos.x,y,other_pos.x,other_y);
        let max_dist = FIGHTER_POISON_DIST_MAX*PostureModule::scale(enemy_boma);
        if dist <= max_dist {
            FIGHTER_POISON_FRAME-=1;
            if FIGHTER_POISON_FRAME <= 0 {
                FIGHTER_POISON_FRAME = FIGHTER_POISON_FRAME_MAX;
                DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_fire_s"), true, false, false, false, enSEType(0));
                let hit_fx = EffectModule::req_on_joint(
                    fighter.module_accessor,
                    Hash40::new("sys_hit_poison"),
                    Hash40::new("rot"),
                    &Vector3f::new(
                        (sv_math::rand(hash40("fighter"), (POISON_FX_DIST_MAX*2.0+1.0) as i32) as f32)-POISON_FX_DIST_MAX, 
                        (sv_math::rand(hash40("fighter"), (POISON_FX_DIST_MAX*2.0+1.0) as i32) as f32)-POISON_FX_DIST_MAX, 
                        (sv_math::rand(hash40("fighter"), (POISON_FX_DIST_MAX*2.0+1.0) as i32) as f32)-POISON_FX_DIST_MAX
                    ),
                    &Vector3f::zero(),
                    1.0,
                    &Vector3f::new(0.1, 0.1, 0.5),
                    &Vector3f::new(0.1, 0.1, 0.5),
                    false,
                    0,
                    0,
                    0
                ) as u32;
                EffectModule::set_rgb(fighter.module_accessor, hit_fx, 1.0, 0.0, 0.0);
            }
        }

    }
}

unsafe fn player_init(fighter: &mut L2CFighterCommon) {
    FIGHTER_POISON_FRAME = 0;
}

unsafe fn respawn_effect(fighter: &mut L2CFighterCommon) {
    ENEMY_EFFECT_ID = EffectModule::req_follow(
        fighter.module_accessor,
        Hash40::new("sys_metamon_aura"), 
        Hash40::new("rot"),
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        &VECTOR_ZERO,
        FIGHTER_POISON_DIST_MAX/1.75,
        true,
    0,0,0,0,0,false,false) as u32;
    EffectModule::set_rgb(fighter.module_accessor, ENEMY_EFFECT_ID, 1.0, 0.0, 0.0);
}

unsafe fn enemy_frame(fighter: &mut L2CFighterCommon) {
    if !EffectModule::is_exist_effect(fighter.module_accessor, ENEMY_EFFECT_ID) {
        respawn_effect(fighter);
    }
}

unsafe fn enemy_init(fighter: &mut L2CFighterCommon) {
    respawn_effect(fighter);
    ENEMY_OBJECT_ID = fighter.battle_object_id;
}

pub unsafe extern "C" fn spirit_frame(fighter: &mut L2CFighterCommon, entry_id: i32) {
    if entry_id == 0 {
        player_frame(fighter);
        if spicy_spirits::is_ready_init() {
            player_init(fighter);
        }
    }
    else {
        enemy_frame(fighter);
        if spicy_spirits::is_ready_init() {
            enemy_init(fighter);
        }
    }
}

