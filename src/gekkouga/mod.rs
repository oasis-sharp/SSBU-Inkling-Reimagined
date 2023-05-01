use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
        
    },
    smash_script::*,
    smashline::*,
    smash::app::utility

};

#[acmd_script( agent = "gekkouga", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn gren_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 68, 40, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.0, 75, 40, 0, 90, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 85, 40, 0, 90, 3.1, 0.0, 2.7, 11.0, Some(0.0), Some(2.5), Some(14.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);

    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_GEKKOUGA )]
fn gren_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        
        let doll_id = WorkModule::get_int(fighter.module_accessor, 0x100000C2);
        let doll_boma = sv_battle_object::module_accessor(doll_id as u32);
        let DOLL_POS = Vector3f {x: PostureModule::pos_x(doll_boma), y: PostureModule::pos_y(doll_boma), z: PostureModule::pos_z(doll_boma)};     
        
        let status = StatusModule::status_kind(fighter.module_accessor);

        if status == *FIGHTER_STATUS_KIND_SPECIAL_N {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
            PostureModule::set_pos(fighter.module_accessor, &DOLL_POS);
            PostureModule::reverse_lr(doll_boma);
        }
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        gren_attackdash
    );

 
    smashline::install_agent_frames!(
        gren_frame
    );

}
