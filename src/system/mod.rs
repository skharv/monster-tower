use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;

mod monster;
mod floor;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, monster::generate)
            .add_systems(PreStartup, monster::load)
            .add_systems(Update, floor::up_down)
        .add_plugins(YamlAssetPlugin::<monster::Monsters>::new(&["yaml"]));
    }
}
