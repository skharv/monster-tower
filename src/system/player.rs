use bevy::prelude::*;

use crate::component;

pub fn spawn(
    mut commands: Commands,
    ) {
    commands.spawn((
            component::Player,
            component::Health {
                max: 100,
                current: 100,
            },
            component::Attack {
                damage: 10,
            },
            component::Armor {
                amount: 5,
            }));
}
