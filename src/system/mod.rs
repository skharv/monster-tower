use bevy::prelude::*;

use crate::AppState;

mod combat;
mod floor;
mod monster;
mod player;

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
