use crate::imports::imports_agent::*;

pub unsafe extern "C" fn player_frame(fighter: &mut L2CFighterCommon) {
    let vec1 = Vector4f{ x: 0.85, y: 0.85, z: 0.85, w: 0.2};
    let vec2 = Vector4f{ x: 0.9907, y: 0.02, z: 0.0251, w: 0.8};
    ColorBlendModule::set_main_color(fighter.module_accessor, &vec1, &vec2, 0.21, 2.2, 5, true);
}
