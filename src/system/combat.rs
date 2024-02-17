use bevy::prelude::*;
use std::fmt;
use rand::Rng;
use crate::AppState;
use crate::component;

#[derive(Debug, Clone, Copy)]
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
    pub physical: i32,
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
            info!("{:?}",monster_name.name);
            *monster_visibility = Visibility::Visible;
        }
    }
}

pub fn exit_combat(
    mut monster_query: Query<(&mut Visibility, &component::Floor, &component::Health, &component::Reward), (With<component::Monster>, Without<component::Player>)>,
    mut player_query: Query<(&mut component::Health, &mut component::Armor, &mut component::Attack, &mut component::Resistance), (With<component::Player>, Without<component::Monster>)>,
    text_query: Query<&component::Floor, (With<Text>, Without<component::Monster>)>,
    ) {
    for floor in text_query.iter() {
        for (mut monster_visibility, monster_floor, monster_health, monster_rewared) in monster_query.iter_mut() {
            if monster_floor.current != floor.current {
                continue;
            }
            if monster_health.current <= 0 {
                info!("You recieved {}", monster_rewared.name);
                for (mut health, mut armor, mut attack, mut resistance) in player_query.iter_mut() {
                    health.current += monster_rewared.health;
                    armor.amount += monster_rewared.armor;
                    attack.damage += monster_rewared.damage;
                    resistance.amount.physical += monster_rewared.physical_resistance;
                    resistance.amount.magic += monster_rewared.magic_resistance;
                    resistance.amount.fire += monster_rewared.fire_resistance;
                    resistance.amount.ice += monster_rewared.ice_resistance;
                    resistance.amount.poison += monster_rewared.poison_resistance;
                    resistance.amount.lightning += monster_rewared.lightning_resistance;
                    resistance.amount.dark += monster_rewared.dark_resistance;
                    resistance.amount.light += monster_rewared.light_resistance;
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
    mut monster_query: Query<(Entity, &component::Name, &mut component::Health, &component::Armor, &component::Attack, &component::Resistance, &component::Floor, &mut component::NextAction), (With<component::Monster>, Without<component::Player>)>,
    mut player_query: Query<(&mut component::Health, &mut component::Mana, &component::Armor, &component::Attack, &component::Fireball, &component::IceSpear, &component::Shock, &component::Resistance), (With<component::Player>, Without<component::Monster>)>,
    mut action_query: Query<(&mut Visibility, Option<&component::PhysicalIcon>, Option<&component::WaitIcon>, Option<&component::DefendIcon>), With<component::Icon>>,
    mut reader: EventReader<ActionEvent>,
    ) {
    let mut rng = rand::thread_rng(); 
    for floor in text_query.iter() {
        for (mut player_health, mut player_mana, player_armor, player_attack, player_fireball, player_icespear, player_shock, player_resisitance) in player_query.iter_mut() {
            for (monster_entity, monster_name, mut monster_health, monster_armor, monster_attack, monster_resistance, monster_floor, mut monster_action) in monster_query.iter_mut() {
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
                            info!("{} waited", monster_name.name);
                        }
                        _ => {
                        }
                    }
                    let mut player_armor_amount = player_armor.amount;
                    match event.0 {
                        ActionType::Attack => {
                            let damage = calculate_damage(player_attack.damage, player_attack.damage_type, monster_armor_amount, monster_resistance.amount);
                            monster_health.current -= damage;
                            info!("You attacked {} for {} damage", monster_name.name, damage);
                            if monster_health.current <= 0 {
                                info!("You defeated {}", monster_name.name);
                                app_state.set(AppState::PostCombat);
                            }
                        }
                        ActionType::Fireball => {
                            if player_mana.current >= player_fireball.mana_cost {
                                let damage = calculate_damage(player_fireball.damage, player_fireball.damage_type, monster_armor_amount, monster_resistance.amount);
                                monster_health.current -= damage;
                                player_mana.current -= player_fireball.mana_cost;
                                info!("You cast Fireball on {} for {} damage", monster_name.name, damage);
                                if monster_health.current <= 0 {
                                    info!("You defeated {}", monster_name.name);
                                    app_state.set(AppState::PostCombat);
                                }
                            } else {
                                info!("You don't have enough mana to cast Fireball");
                            }
                        }
                        ActionType::IceSpear => {
                            if player_mana.current >= player_icespear.mana_cost {
                                let damage = calculate_damage(player_icespear.damage, player_icespear.damage_type, monster_armor_amount, monster_resistance.amount);
                                monster_health.current -= damage;
                                player_mana.current -= player_icespear.mana_cost;
                                info!("You cast Ice Spear on {} for {} damage", monster_name.name, damage);
                                if monster_health.current <= 0 {
                                    info!("You defeated {}", monster_name.name);
                                    app_state.set(AppState::PostCombat);
                                }
                            } else {
                                info!("You don't have enough mana to cast Ice Spear");
                            }
                        }
                        ActionType::Shock => {
                            if player_mana.current >= player_shock.mana_cost {
                                let damage = calculate_damage(player_shock.damage, player_shock.damage_type, monster_armor_amount, monster_resistance.amount);
                                monster_health.current -= damage;
                                player_mana.current -= player_shock.mana_cost;
                                info!("You cast Shock on {} for {} damage", monster_name.name, damage);
                                if monster_health.current <= 0 {
                                    info!("You defeated {}", monster_name.name);
                                    app_state.set(AppState::PostCombat);
                                }
                            } else {
                                info!("You don't have enough mana to cast Shock");
                            }
                        }
                        ActionType::Defend => {
                            player_armor_amount += player_armor.amount;
                        }
                        ActionType::Inspect => {
                            commands.entity(monster_entity).insert(component::Inspected);
                        }
                        ActionType::Wait => {
                            info!("You waited");
                        }
                    }
                    match monster_action.action {
                        ActionType::Attack => {
                            let damage = calculate_damage(monster_attack.damage, monster_attack.damage_type, player_armor_amount, player_resisitance.amount);
                            player_health.current -= damage;
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
    ) {

    if mouse_button.just_pressed(MouseButton::Left) {
        app_state.set(AppState::SelectFloor);
        let mut door_visibility = door_query.single_mut();
        *door_visibility = Visibility::Visible;
    }
}

fn calculate_damage(attacker_damage: i32, attacker_damage_type: DamageType, defender_armor: i32, defender_resistance: Resistances) -> i32 {
    let mut damage = attacker_damage - defender_armor;
    match attacker_damage_type {
        DamageType::Physical => {
            damage -= defender_resistance.physical;
        }
        DamageType::Magic => {
            damage -= defender_resistance.magic;
        }
        DamageType::Fire => {
            damage -= defender_resistance.fire;
        }
        DamageType::Ice => {
            damage -= defender_resistance.ice;
        }
        DamageType::Poison => {
            damage -= defender_resistance.poison;
        }
        DamageType::Lightning => {
            damage -= defender_resistance.lightning;
        }
        DamageType::Dark => {
            damage -= defender_resistance.dark;
        }
        DamageType::Light => {
            damage -= defender_resistance.light;
        }
        DamageType::Ignore => {}
    }
    damage = damage.max(1);
    damage
}
