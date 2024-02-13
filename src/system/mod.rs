use bevy::prelude::*;

use crate::AppState;

mod combat;
mod floor;
mod monster;
mod player;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Attack {
    pub damage: i32,
    pub damage_type: DamageType,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub damage_type: DamageType,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, monster::generate)
            .add_systems(PreStartup, (monster::load, floor::setup, player::spawn))
            .add_systems(Update, floor::up_down.run_if(in_state(AppState::SelectFloor)))
            .add_systems(Update, floor::open_door.run_if(in_state(AppState::OpenDoor)))
            .add_systems(Update, combat::combat.run_if(in_state(AppState::Combat)))
            .add_systems(OnEnter(AppState::Combat), combat::enter_combat)
            .add_systems(OnExit(AppState::Combat), combat::exit_combat);
    }
}
