use bevy::{audio::PlaybackMode, prelude::*, transform::commands};

use crate::{component, AppState};

pub fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut app_state: ResMut<NextState<AppState>>,
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

    let text = "text";
    commands.spawn((
            TextBundle::from_section(
                text, 
                TextStyle {
                    font: asset_server.load("Evil-Empire.otf"),
                    font_size: 100.0,
                    ..default()
                },
                )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            }),
            component::Floor{ current: 0 },
            component::Timer { duration: 5.0 },
            ));
    app_state.set(AppState::SelectFloor);
}

pub fn up_down(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
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
            app_state.set(AppState::OpenDoor);
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
            app_state.set(AppState::OpenDoor);
        }
        text.sections[0].value = floor.current.to_string();
    }
}

pub fn open_door(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>
    ) {
    if keys.just_pressed(KeyCode::Return) {
        commands.spawn(
            AudioBundle{
                source: asset_server.load("teleport.ogg"),
                settings: PlaybackSettings{
                    mode: PlaybackMode::Once,
                    ..default()
                },
                ..default()
            });
        app_state.set(AppState::Combat);
    }
    if keys.just_pressed(KeyCode::Escape) {
        commands.spawn(
            AudioBundle{
                source: asset_server.load("teleport.ogg"),
                settings: PlaybackSettings{
                    mode: PlaybackMode::Once,
                    ..default()
                },
                ..default()
            });
        app_state.set(AppState::SelectFloor);
    }
}
