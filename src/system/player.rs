use bevy::prelude::*;

use crate::component;
use crate::system::combat;

pub fn spawn(
    mut commands: Commands,
    ) {
    commands.spawn((
            component::Player,
            component::Health {
                max: 100,
                current: 100,
            },
            component::Mana {
                max: 100,
                current: 100,
            },
            component::Attack {
                damage: 10,
                damage_type: combat::DamageType::Physical,
            },
            component::Fireball {
                damage: 20,
                damage_type: combat::DamageType::Fire,
                mana_cost: 20,
            },
            component::IceSpear {
                damage: 20,
                damage_type: combat::DamageType::Ice,
                mana_cost: 20,
            },
            component::Shock {
                damage: 20,
                damage_type: combat::DamageType::Lightning,
                mana_cost: 20,
            },
            component::Armor {
                amount: 5,
            },
            component::Resistance {
                amount: combat::Resistances {
                    physical: 0,
                    magic: 0,
                    fire: 0,
                    ice: 0,
                    poison: 0,
                    lightning: 0,
                    dark: 0,
                    light: 0,
                },
            }));
}
