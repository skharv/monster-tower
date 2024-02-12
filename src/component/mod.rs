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
