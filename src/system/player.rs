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
                damage_type: super::DamageType::Physical,
            },
            component::Armor {
                amount: 5,
            },
            component::Resistance {
                physical: 0,
                magic: 0,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            }));
}
