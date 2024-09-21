use crate::imports::imports_agent::*;

pub unsafe extern "C" fn player_frame(fighter: &mut L2CFighterCommon) {
    let vec1 = Vector4f{ x: 0.85, y: 0.85, z: 0.85, w: 0.2};
    let vec2 = Vector4f{ x: 0.5907, y: 0.02, z: 0.0251, w: 0.8};
    ColorBlendModule::set_main_color(fighter.module_accessor, &vec1, &vec2, 0.21, 2.2, 5, true);

    ModelModule::set_scale(fighter.module_accessor, 0.5);
    AttackModule::set_attack_scale(fighter.module_accessor, 0.5, true);
    GrabModule::set_size_mul(fighter.module_accessor, 0.5);
}

pub unsafe extern "C" fn player_init(fighter: &mut L2CFighterCommon) {
    EffectModule::req_on_joint(
        fighter.module_accessor,
        Hash40::new("sys_damage_curse"),
        Hash40::new("top"),
        &Vector3f::new(0.0, 0.0, 0.0),
        &Vector3f::zero(),
        1.5,
        &Vector3f::new(0.1, 0.1, 0.5),
        &Vector3f::new(0.1, 0.1, 0.5),
        false,
        0,
        0,
        0
    );
}
