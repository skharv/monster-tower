use bevy::{audio::PlaybackMode, prelude::*, text::{BreakLineOn, Text2dBounds}, transform::commands};
use rand::Rng;

use crate::{component::{self, FloorSelector, FloorVisualizer}, AppState};

pub const ELEVATOR_SPEED: f32 = 1.0;

pub fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>
    ) {
    let texture_handle = asset_server.load("dungeon_room.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(crate::WIDTH as f32, crate::HEIGHT as f32), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

    let texture_handle = asset_server.load("dungeon_elevator.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(crate::WIDTH as f32, crate::HEIGHT as f32), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    });

    let texture_handle = asset_server.load("dungeon_door.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(crate::WIDTH as f32, crate::HEIGHT as f32), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0., 0., 2.),
        ..default()
    },
    component::Door));
    
    let texture_handle = asset_server.load("dungeon_floor_number.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(crate::WIDTH as f32, crate::HEIGHT as f32), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0., 0., 3.),
        ..default()
    });

    let text = "G";
    commands.spawn((
            TextBundle::from_section(
                text, 
                TextStyle {
                    font: asset_server.load("Evil-Empire.otf"),
                    font_size: 40.0,
                    ..default()
                },
                )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(310.0),
                right: Val::Px(55.0),
                ..default()
            }),
            component::Floor{ current: 0 },
            component::FloorSelector
     ));

    commands.spawn((
            TextBundle::from_section(
                text, 
                TextStyle {
                    font: asset_server.load("Evil-Empire.otf"),
                    font_size: 80.0,
                    ..default()
                },
                )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(15.0),
                left: Val::Px(385.0),
                ..default()
            }),
            component::Floor{ current: 0 },
            component::Timer{ duration: ELEVATOR_SPEED },
            component::FloorVisualizer
            ));
    let box_size = Vec2::new(600.0, 100.0);
    let box_position = Vec2::new(0.0, -200.0);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.75),
            custom_size: Some(box_size),
            ..default()
        },
        transform: Transform::from_xyz(box_position.x, box_position.y, 10.0),
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::DescriptionBox)
    .with_children(|builder| {
        builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Press J to go down".to_string(),
                            style: TextStyle {
                                font: asset_server.load("Evil-Empire.otf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        }],
                        alignment: TextAlignment::Center,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    size: box_size,
                },
                transform: Transform::from_translation(Vec3::Z),
                ..default()
        })
        .insert(component::DescriptionText);
    });

    app_state.set(AppState::SelectFloor);
}

pub fn up_down(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut component::Floor, &mut Text), With<FloorSelector>>,
    ) {
    for (mut floor, mut text) in query.iter_mut() {
        if keys.just_pressed(KeyCode::K) {
            floor.current += 1;
        }
        if keys.just_pressed(KeyCode::J) {
            floor.current -= 1;
        }
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
            app_state.set(AppState::MoveFloor);
        }
        let mut floor_text = floor.current.to_string();
        if floor.current == 0 {
            floor_text = "G".to_string();
        }
        text.sections[0].value = floor_text;
    }
}

pub fn move_floors(
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut visualizer_query: Query<(&mut component::Floor, &mut component::Timer, &mut Text), (With<FloorVisualizer>, Without<FloorSelector>)>,
    selector_query: Query<&component::Floor, (With<FloorSelector>, Without<FloorVisualizer>)>,
    ) {
    let (mut floor, mut timer, mut text) = visualizer_query.single_mut();
    let selector = selector_query.single(); 
    
    timer.duration -= time.delta_seconds();
    if timer.duration <= 0.0 {
        if selector.current > floor.current {
            floor.current += 1;
            let mut floor_text = floor.current.to_string();
            if floor.current == 0 {
                floor_text = "G".to_string();
            }
            text.sections[0].value = floor_text;
            timer.duration = ELEVATOR_SPEED;
        } else if selector.current < floor.current {
            floor.current -= 1;
            let mut floor_text = floor.current.to_string();
            if floor.current == 0 {
                floor_text = "G".to_string();
            }
            text.sections[0].value = floor_text;
            timer.duration = ELEVATOR_SPEED;
        } else {
            app_state.set(AppState::OpenDoor);
        }
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

pub fn set_and_show_description(
    mut box_query: Query<&mut Visibility, With<component::DescriptionBox>>,
    mut text_query: Query<&mut Text, With<component::DescriptionText>>,
    monster_query: Query<(&component::Description, &component::Floor), With<component::Monster>>,
    floor_query: Query<&component::Floor, (Without<component::Monster>, With<component::FloorSelector>)>,
    ) {
    let mut rng = rand::thread_rng();
    let mut box_visibility = box_query.single_mut();
    let mut text = text_query.single_mut();
    let floor = floor_query.single();
    for (monster_description, monster_floor) in monster_query.iter() {
        if monster_floor.current != floor.current {
            continue;
        }
        *box_visibility = Visibility::Visible;
        text.sections[0].value = monster_description.descriptions[rng.gen_range(0..3)].clone();
    }
}


pub fn hide_description(
    mut box_query: Query<&mut Visibility, With<component::DescriptionBox>>,
    ) {
    let mut box_visibility = box_query.single_mut();
    *box_visibility = Visibility::Hidden;
}
