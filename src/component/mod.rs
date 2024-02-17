use bevy::prelude::*;
use crate::system::combat;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerStatsText;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Door;

#[derive(Component)]
pub struct CombatUi;

#[derive(Component)]
pub struct UpButton;

#[derive(Component)]
pub struct DownButton;

#[derive(Component)]
pub struct FloorSelector;

#[derive(Component)]
pub struct FloorVisualizer;

#[derive(Component)]
pub struct Floor {
    pub current: i32,
}

#[derive(Component)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}

#[derive(Component)]
pub struct Mana {
    pub max: i32,
    pub current: i32,
}

#[derive(Component)]
pub struct Attack {
    pub damage: i32,
    pub damage_type: combat::DamageType,
}

#[derive(Component)]
pub struct Fireball {
    pub damage: i32,
    pub damage_type: combat::DamageType,
    pub mana_cost: i32,
}

#[derive(Component)]
pub struct IceSpear {
    pub damage: i32,
    pub damage_type: combat::DamageType,
    pub mana_cost: i32,
}

#[derive(Component)]
pub struct Shock {
    pub damage: i32,
    pub damage_type: combat::DamageType,
    pub mana_cost: i32,
}

#[derive(Component)]
pub struct Resistance {
    pub amount: combat::Resistances
}

#[derive(Component)]
pub struct Reward {
    pub name: String,
    pub physical_resistance: i32,
    pub magic_resistance: i32,
    pub fire_resistance: i32,
    pub ice_resistance: i32,
    pub poison_resistance: i32,
    pub lightning_resistance: i32,
    pub dark_resistance: i32,
    pub light_resistance: i32,
    pub health: i32,
    pub damage: i32,
    pub armor: i32,
    pub damage_type: combat::DamageType,
}

#[derive(Component)]
pub struct Description {
    pub descriptions: [String; 3],
}

#[derive(Component)]
pub struct DescriptionText;

#[derive(Component)]
pub struct DescriptionBox;

#[derive(Component)]
pub struct Armor {
    pub amount: i32,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Timer {
    pub duration: f32,
}
