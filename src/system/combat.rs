use bevy::prelude::*;
use std::fmt;
use rand::Rng;
use crate::AppState;
use crate::component;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DamageType {
    Physical,
    Magic,
    Fire,
    Ice,
    Poison,
    Lightning,
    Dark,
    Light,
    Ignore
}

impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DamageType::Physical => write!(f, "Physical"),
            DamageType::Magic => write!(f, "Magic"),
            DamageType::Fire => write!(f, "Fire"),
            DamageType::Ice => write!(f, "Ice"),
            DamageType::Poison => write!(f, "Poison"),
            DamageType::Lightning => write!(f, "Lightning"),
            DamageType::Dark => write!(f, "Dark"),
            DamageType::Light => write!(f, "Light"),
            DamageType::Ignore => write!(f, "Ignore"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Attack {
    pub damage: i32,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone, Copy)]
pub struct Resistances {
    pub magic: i32,
    pub fire: i32,
    pub ice: i32,
    pub poison: i32,
    pub lightning: i32,
    pub dark: i32,
    pub light: i32,
}

pub enum ActionType {
    Attack,
    Fireball,
    IceSpear,
    Shock,
    Defend,
    Inspect,
    Wait,
}

#[derive(Event)]
pub struct ActionEvent(pub ActionType);

pub fn enter_combat(
    mut monster_query: Query<(&mut Visibility, &component::Floor, &component::Name, &component::Health), With<component::Monster>>,
    mut door_query: Query<&mut Visibility, (With<component::Door>, Without<component::Monster>)>,
    text_query: Query<&component::Floor, (With<Text>, Without<component::Monster>)>,
    ) {
    let mut door_visibility = door_query.single_mut();
    *door_visibility = Visibility::Hidden;

    for floor in text_query.iter() {
        for (mut monster_visibility, monster_floor, monster_name, monster_health) in monster_query.iter_mut() {
            if monster_floor.current != floor.current {
                continue;
            }
            if monster_health.current <= 0 {
                continue;
            }
            *monster_visibility = Visibility::Visible;
        }
    }
}

pub fn exit_combat(
    mut monster_query: Query<(&mut Visibility, &component::Floor, &component::Health, &mut component::Reward), (With<component::Monster>, Without<component::Player>)>,
    mut player_query: Query<(&mut component::Health, &mut component::Armor, &mut component::Attack, &mut component::Resistance), (With<component::Player>, Without<component::Monster>)>,
    mut reward_query: Query<&mut Text, With<component::RewardText>>,
    text_query: Query<&component::Floor, (With<Text>, Without<component::Monster>)>,
    ) {
    let mut reward = reward_query.single_mut();
    for floor in text_query.iter() {
        for (mut monster_visibility, monster_floor, monster_health, mut monster_reward) in monster_query.iter_mut() {
            if monster_floor.current != floor.current {
                continue;
            }
            if monster_health.current <= 0 {
                if let Some(obtained_reward) = &monster_reward.reward {
                    reward.sections[0].value = format!("You've obtained {}", obtained_reward);
                    for (mut health, mut armor, mut attack, mut resistance) in player_query.iter_mut() {
                        health.current += obtained_reward.health;
                        health.max += obtained_reward.health;
                        armor.amount += obtained_reward.armor;
                        attack.damage += obtained_reward.damage;
                        if obtained_reward.damage_type != DamageType::Ignore {
                            attack.damage_type = obtained_reward.damage_type;
                        }
                        resistance.amount.magic += obtained_reward.magic_resistance;
                        resistance.amount.fire += obtained_reward.fire_resistance;
                        resistance.amount.ice += obtained_reward.ice_resistance;
                        resistance.amount.poison += obtained_reward.poison_resistance;
                        resistance.amount.lightning += obtained_reward.lightning_resistance;
                        resistance.amount.dark += obtained_reward.dark_resistance;
                        resistance.amount.light += obtained_reward.light_resistance;
                    }
                    monster_reward.reward = None;
                }
            }
            *monster_visibility = Visibility::Hidden;
        }
    }
}

pub fn combat(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    text_query: Query<&component::Floor, (With<Text>, Without<component::Monster>)>,
    mut monster_query: Query<(Entity, &mut TextureAtlasSprite, &component::Name, &mut component::Health, &component::Armor, &component::Attack, &component::Resistance, &component::Floor, &mut component::NextAction), (With<component::Monster>, Without<component::Player>)>,
    mut player_query: Query<(&mut component::Health, &mut component::Mana, &component::Armor, &component::Attack, &component::Fireball, &component::IceSpear, &component::Shock, &component::Resistance), (With<component::Player>, Without<component::Monster>)>,
    mut action_query: Query<(&mut Visibility, Option<&component::PhysicalIcon>, Option<&component::WaitIcon>, Option<&component::DefendIcon>), With<component::Icon>>,
    mut reader: EventReader<ActionEvent>,
    ) {
    let mut rng = rand::thread_rng(); 
    for floor in text_query.iter() {
        for (mut player_health, mut player_mana, player_armor, player_attack, player_fireball, player_icespear, player_shock, player_resisitance) in player_query.iter_mut() {
            for (monster_entity, mut monster_sprite, monster_name, mut monster_health, monster_armor, monster_attack, monster_resistance, monster_floor, mut monster_action) in monster_query.iter_mut() {
                if monster_floor.current != floor.current {
                    continue;
                }
                for event in reader.read() {
                    let mut monster_armor_amount = monster_armor.amount;
                    match monster_action.action {
                        ActionType::Defend => {
                            monster_armor_amount += monster_armor.amount;
                        }
                        ActionType::Wait => {
                        }
                        _ => {
                        }
                    }

                    let mut player_armor_amount = player_armor.amount;
                    match event.0 {
                        ActionType::Attack => {
                            let damage = calculate_damage(player_attack.damage, player_attack.damage_type, monster_armor_amount, monster_resistance.amount);
                            monster_health.current -= damage;
                            if monster_health.current <= 0 {
                                app_state.set(AppState::PostCombat);
                            }
                            monster_sprite.color = Color::RED;
                        }
                        ActionType::Fireball => {
                            if player_mana.current >= player_fireball.mana_cost {
                                let damage = calculate_damage(player_fireball.damage, player_fireball.damage_type, monster_armor_amount, monster_resistance.amount);
                                monster_health.current -= damage;
                                player_mana.current -= player_fireball.mana_cost;
                                if monster_health.current <= 0 {
                                    app_state.set(AppState::PostCombat);
                                }
                                monster_sprite.color = Color::RED;
                            } else {
                            }
                        }
                        ActionType::IceSpear => {
                            if player_mana.current >= player_icespear.mana_cost {
                                let damage = calculate_damage(player_icespear.damage, player_icespear.damage_type, monster_armor_amount, monster_resistance.amount);
                                monster_health.current -= damage;
                                player_mana.current -= player_icespear.mana_cost;
                                if monster_health.current <= 0 {
                                    app_state.set(AppState::PostCombat);
                                }
                                monster_sprite.color = Color::RED;
                            } else {
                            }
                        }
                        ActionType::Shock => {
                            if player_mana.current >= player_shock.mana_cost {
                                let damage = calculate_damage(player_shock.damage, player_shock.damage_type, monster_armor_amount, monster_resistance.amount);
                                monster_health.current -= damage;
                                player_mana.current -= player_shock.mana_cost;
                                if monster_health.current <= 0 {
                                    app_state.set(AppState::PostCombat);
                                }
                                monster_sprite.color = Color::RED;
                            } else {
                            }
                        }
                        ActionType::Defend => {
                            player_armor_amount += player_armor.amount;
                        }
                        ActionType::Inspect => {
                            commands.entity(monster_entity).insert(component::Inspected);
                        }
                        ActionType::Wait => {
                        }
                    }
                    match monster_action.action {
                        ActionType::Attack => {
                            let damage = calculate_damage(monster_attack.damage, monster_attack.damage_type, player_armor_amount, player_resisitance.amount);
                            player_health.current -= damage;
                            if player_health.current <= 0 {
                                app_state.set(AppState::Loss);
                            }
                        }
                        _ => {
                        }
                    }
                    let action = rng.gen_range(0..10);
                    if action > 5 {
                        for (mut sprite, physical_icon, _, _) in action_query.iter_mut() {
                            if let Some(_) = physical_icon {
                                *sprite = Visibility::Visible;
                            } else {
                                *sprite = Visibility::Hidden;
                            }
                        }
                        monster_action.action = ActionType::Attack;
                    } else if action > 2 {
                        for (mut sprite, _, _, defend_icon) in action_query.iter_mut() {
                            if let Some(_) = defend_icon {
                                *sprite = Visibility::Visible;
                            } else {
                                *sprite = Visibility::Hidden;
                            }
                        }
                        monster_action.action = ActionType::Defend;
                    } else {
                        for (mut sprite, _, wait_icon, _) in action_query.iter_mut() {
                            if let Some(_) = wait_icon {
                                *sprite = Visibility::Visible;
                            } else {
                                *sprite = Visibility::Hidden;
                            }
                        }
                        monster_action.action = ActionType::Wait;
                    }
                }
            }
        }
    }
}

pub fn post_combat(
    mouse_button: Res<Input<MouseButton>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut door_query: Query<&mut Visibility, (With<component::Door>, Without<component::Monster>)>,
    mut player_query: Query<(&mut component::Health, &mut component::Mana), (With<component::Player>, Without<component::Monster>)>,
    monster_query: Query<&component::Health, With<component::Monster>>,
    ) {
    if mouse_button.just_pressed(MouseButton::Left) {
        for (mut health, mut mana) in player_query.iter_mut() {
            let half_health_missing = (health.max - health.current) / 2;
            health.current += half_health_missing;
            let half_mana_missing = (mana.max - mana.current) / 2;
            mana.current += half_mana_missing;
        } 

        let mut victory = true;
        for health in monster_query.iter() {
            if health.current > 0 {
                victory = false;
            }
        }

        if victory {
            app_state.set(AppState::Win);
        } else {
            app_state.set(AppState::SelectFloor);
        }

        let mut door_visibility = door_query.single_mut();
        *door_visibility = Visibility::Visible;
    }
}

fn calculate_damage(attacker_damage: i32, attacker_damage_type: DamageType, defender_armor: i32, defender_resistance: Resistances) -> i32 {
    let mut damage = attacker_damage;
    match attacker_damage_type {
        DamageType::Physical => {
            damage -= defender_armor;
        }
        DamageType::Magic => {
            damage -= defender_resistance.magic;
        }
        DamageType::Fire => {
            damage -= defender_resistance.fire + defender_resistance.magic;
        }
        DamageType::Ice => {
            damage -= defender_resistance.ice + defender_resistance.magic;
        }
        DamageType::Poison => {
            damage -= defender_resistance.poison + defender_resistance.magic;
        }
        DamageType::Lightning => {
            damage -= defender_resistance.lightning + defender_resistance.magic;
        }
        DamageType::Dark => {
            damage -= defender_resistance.dark + defender_resistance.magic;
        }
        DamageType::Light => {
            damage -= defender_resistance.light + defender_resistance.magic;
        }
        DamageType::Ignore => {}
    }
    damage = damage.max(1);
    damage
}
