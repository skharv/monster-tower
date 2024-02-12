use bevy::prelude::*;

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
pub struct Attack {
    pub damage: i32,
}

#[derive(Component)]
pub struct Armor {
    pub amount: i32,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}
