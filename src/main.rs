use bevy::{prelude::*, window::WindowResolution, asset::AssetMetaCheck};

mod system;
mod component;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 800;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Start,
    SelectFloor,
    MoveFloor,
    OpenDoor,
    Combat,
    PostCombat,
    Win,
    Loss
}

pub static mut STATE: AppState = AppState::Start;

fn main() {
    let mut app = App::new();

    app.insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "monster-tower".into(),
            resolution: WindowResolution::new(
                WIDTH as f32,
                HEIGHT as f32
                ),
                resize_constraints: WindowResizeConstraints {
                    min_width: WIDTH as f32,
                min_height: HEIGHT as f32, 
                ..default()
                },
                fit_canvas_to_parent: false,
                ..default()
        }),
        close_when_requested: true,
        exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
    }).set(AssetPlugin {
        mode: AssetMode::Unprocessed,
        ..default()
    }))
    .add_plugins(system::GamePlugin)
        .add_state::<AppState>()
        .init_asset::<AudioSource>()
    .run();
}
