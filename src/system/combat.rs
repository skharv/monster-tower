use bevy::prelude::*;

use crate::AppState;
use crate::component;

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
                    resistance.physical += monster_rewared.physical_resistance;
                    resistance.magic += monster_rewared.magic_resistance;
                    resistance.fire += monster_rewared.fire_resistance;
                    resistance.ice += monster_rewared.ice_resistance;
                    resistance.poison += monster_rewared.poison_resistance;
                    resistance.lightning += monster_rewared.lightning_resistance;
                    resistance.dark += monster_rewared.dark_resistance;
                    resistance.light += monster_rewared.light_resistance;
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
    mut monster_query: Query<(&component::Name, &mut component::Health, &component::Armor, &component::Attack, &component::Resistance, &component::Floor), (With<component::Monster>, Without<component::Player>)>,
    mut player_query: Query<(&mut component::Health, &component::Armor, &component::Attack, &component::Resistance), (With<component::Player>, Without<component::Monster>)>,
    keys: Res<Input<KeyCode>>,
    ) {
    for floor in text_query.iter() {
        for (mut player_health, player_armor, player_attack, player_resisitance) in player_query.iter_mut() {
            for (monster_name, mut monster_health, monster_armor, monster_attack, monster_resistance, monster_floor) in monster_query.iter_mut() {
                if monster_floor.current != floor.current {
                    continue;
                }
                if keys.just_pressed(KeyCode::Return) {
                    let mut damage = player_attack.damage - monster_armor.amount;
                    match player_attack.damage_type {
                        super::DamageType::Physical => {
                            damage -= monster_resistance.physical;
                        }
                        super::DamageType::Magic => {
                            damage -= monster_resistance.magic;
                        }
                        super::DamageType::Fire => {
                            damage -= monster_resistance.fire;
                        }
                        super::DamageType::Ice => {
                            damage -= monster_resistance.ice;
                        }
                        super::DamageType::Poison => {
                            damage -= monster_resistance.poison;
                        }
                        super::DamageType::Lightning => {
                            damage -= monster_resistance.lightning;
                        }
                        super::DamageType::Dark => {
                            damage -= monster_resistance.dark;
                        }
                        super::DamageType::Light => {
                            damage -= monster_resistance.light;
                        }
                        super::DamageType::Ignore => {}
                    }
                    damage = damage.max(1);
                    info!("You hit the {} for {} damage", monster_name.name, damage);
                    monster_health.current -= damage;
                    let mut monster_damage = monster_attack.damage - player_armor.amount;
                    match monster_attack.damage_type {
                        super::DamageType::Physical => {
                            monster_damage -= player_resisitance.physical;
                        }
                        super::DamageType::Magic => {
                            monster_damage -= player_resisitance.magic;
                        }
                        super::DamageType::Fire => {
                            monster_damage -= player_resisitance.fire;
                        }
                        super::DamageType::Ice => {
                            monster_damage -= player_resisitance.ice;
                        }
                        super::DamageType::Poison => {
                            monster_damage -= player_resisitance.poison;
                        }
                        super::DamageType::Lightning => {
                            monster_damage -= player_resisitance.lightning;
                        }
                        super::DamageType::Dark => {
                            monster_damage -= player_resisitance.dark;
                        }
                        super::DamageType::Light => {
                            monster_damage -= player_resisitance.light;
                        }
                        super::DamageType::Ignore => {}
                    }
                    monster_damage = monster_damage.max(1);
                    info!("The {} hit you for {} damage", monster_name.name, monster_damage);
                    player_health.current -= monster_damage;
                    if player_health.current <= 0 {
                        info!("You died");
                    }
                    if monster_health.current <= 0 {
                        info!("You killed the {}", monster_name.name);
                        app_state.set(AppState::PostCombat);
                    }
                }
            }
        }
    }
}

pub fn post_combat(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut door_query: Query<&mut Visibility, (With<component::Door>, Without<component::Monster>)>,
    ) {
    if keys.just_pressed(KeyCode::Return) {
        app_state.set(AppState::SelectFloor);
        let mut door_visibility = door_query.single_mut();
        *door_visibility = Visibility::Visible;
    }
}
