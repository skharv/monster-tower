use bevy::prelude::*;

use crate::AppState;
use crate::component;

pub fn enter_combat(
    mut monster_query: Query<(&mut Visibility, &component::Floor, &component::Name), With<component::Monster>>,
    text_query: Query<&component::Floor, (With<Text>, Without<component::Monster>)>,
    ) {
    for floor in text_query.iter() {
        for (mut monster_visibility, monster_floor, monster_name) in monster_query.iter_mut() {
            if monster_floor.current != floor.current {
                continue;
            }
            info!("{:?}",monster_name.name);
            *monster_visibility = Visibility::Visible;
        }
    }
}

pub fn exit_combat(
    mut monster_query: Query<(&mut Visibility, &component::Floor), With<component::Monster>>,
    text_query: Query<&component::Floor, (With<Text>, Without<component::Monster>)>,
    ) {
    for floor in text_query.iter() {
        for (mut monster_visibility, monster_floor) in monster_query.iter_mut() {
            if monster_floor.current != floor.current {
                continue;
            }
            *monster_visibility = Visibility::Hidden;
        }
    }
}

pub fn combat(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    text_query: Query<&component::Floor, (With<Text>, Without<component::Monster>)>,
    mut monster_query: Query<(&component::Name, &mut component::Health, &component::Armor, &component::Attack, &component::Floor), (With<component::Monster>, Without<component::Player>)>,
    mut player_query: Query<(&mut component::Health, &component::Armor, &component::Attack), (With<component::Player>, Without<component::Monster>)>,
    keys: Res<Input<KeyCode>>,
    ) {
    for floor in text_query.iter() {
        for (mut health, armor, attack) in player_query.iter_mut() {
            for (monster_name, mut monster_health, monster_armor, monster_attack, monster_floor) in monster_query.iter_mut() {
                if monster_floor.current != floor.current {
                    continue;
                }
                if keys.just_pressed(KeyCode::Return) {
                    let mut damage = attack.damage - monster_armor.amount;
                    damage = damage.max(1);
                    info!("You hit the {} for {} damage", monster_name.name, damage);
                    monster_health.current -= damage;
                    if monster_health.current <= 0 {
                        info!("You killed the {}", monster_name.name);
                        app_state.set(AppState::SelectFloor);
                    }
                }
            }
        }
    }
}
