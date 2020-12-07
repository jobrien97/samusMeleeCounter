use smash::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use smash::app::lua_bind::*;
use smash::lib::L2CValueType;
use smash::phx::Vector3f;
use smash::phx::Hash40;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_SAMUS,
    animation = "appeal_lw_r",
    animcmd = "game_appeallwr")]
pub fn samus_counter(fighter: &mut L2CFighterCommon) {
    let mut globals = *fighter.globals_mut();

    if let L2CValueType::Void = globals["damage"].val_type {
        globals["damage"] = 0.0.into();
    }
    if let L2CValueType::Void = globals["prev_damage"].val_type {
        globals["prev_damage"] = 0.0.into();
    }

    if MotionModule::frame(module_accessor) == 2.0 {
        globals["prev_damage"] = DamageModule::damage(module_accessor, 0).into();
    }

    acmd!({
        frame(3)
        if(is_excute) {
            sv_animcmd::PLAY_SE(hash40("se_samus_win1"))
        }
        frame(4)
        if(is_excute) {
            sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_flash"), /*Bone*/ hash40("top"), /*X*/ -5.0, /*Y*/ 13.0, /*Z*/ 3.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.65, true)
            sv_animcmd::LAST_EFFECT_SET_COLOR(0.1, 0.7, 3.0)
            sv_animcmd::LAST_EFFECT_SET_RATE(1.0)
        }
    });

    if MotionModule::frame(module_accessor) == 5.0 {
        acmd!(lua_state, {
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
        });
    }

    if MotionModule::frame(module_accessor) > 5.0 && MotionModule::frame(module_accessor) < 22.0 {
        globals["damage"] = DamageModule::damage(module_accessor, 0).into();
        if globals["damage"].get_num() != globals["prev_damage"].get_num() {
            acmd!({
                sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_counter_flash"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.0, true)
            });
            let dif = globals["damage"].get_num() - globals["prev_damage"].get_num();
            DamageModule::add_damage(module_accessor, (dif * -1.0), 0);
            globals["prev_damage"] = globals["damage"].get_num().into();
            acmd!(lua_state, {
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
            });
            MotionModule::set_frame(module_accessor, 65.0, false);
        }
    }
    if globals["damage"].get_num() == globals["prev_damage"].get_num() {
        if MotionModule::frame(module_accessor) == 45.0 {
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }

    acmd!({
        frame(65)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
        }
        frame(70)
        if(is_excute){
            sv_animcmd::PLAY_SE(hash40("se_samus_swing_l"))
            sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_attack_arc_b"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 4.0, /*XRot*/ 0, /*YRot*/ 20, /*ZRot*/ 90, /*Size?*/ 1.35, true)
            sv_animcmd::LAST_EFFECT_SET_COLOR(/*R*/ 0.1, /*G*/ 0.7, /*B*/ 3.0)
            sv_animcmd::LAST_EFFECT_SET_RATE(0.45)
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=4.0, Angle=70, KBG=100, FKB=55, BKB=0, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=4.0, Angle=75, KBG=100, FKB=55, BKB=0, Size=5.0, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
            AttackModule::set_add_reaction_frame(0, 15.0, false)
            AttackModule::set_add_reaction_frame(1, 15.0, false)
        }
        frame(74)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(94)
        if(is_excute){
            MotionModule::set_frame(0.0, false)
            ArticleModule::generate_article(*FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, 0)
            StatusModule::change_status_request_from_script(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, true)
        }
        frame(138)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_SAMUS,
    animation = "appeal_lw_r",
    animcmd = "effect_appeallwr")]
pub fn samus_counter_eff(fighter: &mut L2CFighterCommon) {
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_SAMUS,
    animation = "appeal_lw_r",
    animcmd = "sound_appeallwr")]
pub fn samus_counter_sound(fighter: &mut L2CFighterCommon) {
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_SAMUS,
    animation = "appeal_lw_l",
    animcmd = "game_appeallwl")]
pub fn samus_counter_l(fighter: &mut L2CFighterCommon) {
    let mut globals = *fighter.globals_mut();

    if let L2CValueType::Void = globals["damage"].val_type {
        globals["damage"] = 0.0.into();
    }
    if let L2CValueType::Void = globals["prev_damage"].val_type {
        globals["prev_damage"] = 0.0.into();
    }

    if MotionModule::frame(module_accessor) == 2.0 {
        globals["prev_damage"] = DamageModule::damage(module_accessor, 0).into();
    }

    acmd!({
        frame(3)
        if(is_excute) {
            sv_animcmd::PLAY_SE(hash40("se_samus_win1"))
        }
        frame(4)
        if(is_excute) {
            sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_flash"), /*Bone*/ hash40("top"), /*X*/ 5.0, /*Y*/ 13.0, /*Z*/ 3.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.65, true)
            sv_animcmd::LAST_EFFECT_SET_COLOR(0.1, 0.7, 3.0)
            sv_animcmd::LAST_EFFECT_SET_RATE(1.0)
        }
    });

    if MotionModule::frame(module_accessor) == 5.0 {
        acmd!(lua_state, {
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
        });
    }

    if MotionModule::frame(module_accessor) > 5.0 && MotionModule::frame(module_accessor) < 22.0 {
        globals["damage"] = DamageModule::damage(module_accessor, 0).into();
        if globals["damage"].get_num() != globals["prev_damage"].get_num() {
            acmd!({
                sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_counter_flash"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 1.0, true)
            });
            let dif = globals["damage"].get_num() - globals["prev_damage"].get_num();
            DamageModule::add_damage(module_accessor, (dif * -1.0), 0);
            globals["prev_damage"] = globals["damage"].get_num().into();
            acmd!(lua_state, {
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
            });
            MotionModule::set_frame(module_accessor, 65.0, false);
        }
    }
    if globals["damage"].get_num() == globals["prev_damage"].get_num() {
        if MotionModule::frame(module_accessor) == 45.0 {
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }

    acmd!({
        frame(65)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
        }
        frame(70)
        if(is_excute){
            sv_animcmd::PLAY_SE(hash40("se_samus_swing_l"))
            sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_attack_arc_b"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 4.0, /*XRot*/ 0, /*YRot*/ 20, /*ZRot*/ 90, /*Size?*/ 1.35, true)
            sv_animcmd::LAST_EFFECT_SET_COLOR(/*R*/ 0.1, /*G*/ 0.7, /*B*/ 3.0)
            sv_animcmd::LAST_EFFECT_SET_RATE(0.45)
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=4.0, Angle=70, KBG=100, FKB=55, BKB=0, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=4.0, Angle=75, KBG=100, FKB=55, BKB=0, Size=5.0, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_attack_height_all(app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
            AttackModule::set_add_reaction_frame(0, 15.0, false)
            AttackModule::set_add_reaction_frame(1, 15.0, false)
        }
        frame(74)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(94)
        if(is_excute){
            MotionModule::set_frame(0.0, false)
            ArticleModule::generate_article(*FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, 0)
            StatusModule::change_status_request_from_script(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, true)
        }
        frame(138)
        if(is_excute){
            HitModule::set_whole(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_SAMUS,
    animation = "appeal_lw_l",
    animcmd = "effect_appeallwl")]
pub fn samus_counter_l_eff(fighter: &mut L2CFighterCommon) {
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_SAMUS,
    animation = "appeal_lw_l",
    animcmd = "sound_appeallwl")]
pub fn samus_counter_l_sound(fighter: &mut L2CFighterCommon) {
}


pub fn install() {
    acmd::add_hooks!(
        samus_counter,
        samus_counter_eff,
        samus_counter_sound,
        samus_counter_l,
        samus_counter_l_eff,
        samus_counter_l_sound
    );
}
