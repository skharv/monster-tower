use bevy::prelude::*;

use crate::AppState;

mod combat;
mod floor;
mod monster;
mod player;
mod reward;
mod ui;

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


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, monster::generate)
            .add_systems(PreStartup, (monster::load, floor::setup, player::spawn))
            .add_systems(Update, floor::up_down.run_if(in_state(AppState::SelectFloor)))
            .add_systems(Update, floor::open_door.run_if(in_state(AppState::OpenDoor)))
            .add_systems(Update, combat::combat.run_if(in_state(AppState::Combat)))
            .add_systems(Update, combat::post_combat.run_if(in_state(AppState::PostCombat)))
            .add_systems(Update, floor::move_floors.run_if(in_state(AppState::MoveFloor)))
            .add_systems(OnEnter(AppState::OpenDoor), floor::set_and_show_description)
            .add_systems(OnExit(AppState::OpenDoor), floor::hide_description)
            .add_systems(OnEnter(AppState::Combat), combat::enter_combat)
            .add_systems(OnExit(AppState::Combat), combat::exit_combat);
    }
}
