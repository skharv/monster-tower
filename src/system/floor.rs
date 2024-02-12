use bevy::{audio::PlaybackMode, prelude::*};

use crate::component;

pub fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>
    ) {
    let texture_handle = asset_server.load("test.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1024., 1024.), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

}

pub fn up_down(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut component::Floor, &mut Text)>,
    ) {
    for (mut floor, mut text) in query.iter_mut() {
        if keys.just_pressed(KeyCode::K) {
            floor.current += 1;
            commands.spawn(
                AudioBundle{
                    source: asset_server.load("teleport.ogg"),
                    settings: PlaybackSettings{
                        mode: PlaybackMode::Once,
                        ..default()
                    },
                    ..default()
                });
        }
        if keys.just_pressed(KeyCode::J) {
            floor.current -= 1;
            commands.spawn(
                AudioBundle{
                    source: asset_server.load("teleport.ogg"),
                    settings: PlaybackSettings{
                        mode: PlaybackMode::Once,
                        ..default()
                    },
                    ..default()
                });
        }
        text.sections[0].value = floor.current.to_string();
    }
}
