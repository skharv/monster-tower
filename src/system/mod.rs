use bevy::prelude::*;

use crate::AppState;

pub mod combat;
mod floor;
mod monster;
mod player;
mod reward;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (monster::generate, ui::setup_elevator, ui::setup_combat, ui::setup_monster_actions))
            .add_systems(PreStartup, (monster::load, floor::setup, player::spawn))
            .add_systems(PostStartup, (ui::setup_monster_stats, ui::setup_player_stats))
            .add_systems(Update, floor::up_down.run_if(in_state(AppState::SelectFloor)))
            .add_systems(Update, ui::navigation_buttons.run_if(in_state(AppState::SelectFloor)))
            .add_systems(Update, floor::open_door.run_if(in_state(AppState::OpenDoor)))
            .add_systems(Update, (combat::combat, ui::combat_buttons).run_if(in_state(AppState::Combat)))
            .add_systems(Update, combat::post_combat.run_if(in_state(AppState::PostCombat)))
            .add_systems(Update, floor::move_floors.run_if(in_state(AppState::MoveFloor)))
            .add_systems(Update, (ui::update_player_stats, ui::show_monster_stats, ui::update_monster_stats))
            .add_systems(OnEnter(AppState::OpenDoor), ui::set_and_show_description)
            .add_systems(OnExit(AppState::OpenDoor), ui::hide_description)
            .add_systems(OnEnter(AppState::Combat), (combat::enter_combat, ui::show_combat))
            .add_systems(OnExit(AppState::Combat), (combat::exit_combat, ui::hide_combat))
            .add_event::<floor::SetFloor>()
            .add_event::<combat::ActionEvent>();
    }
}
