use bevy::{audio::PlaybackMode, prelude::*};

use crate::{component::{self, FloorSelector, FloorVisualizer}, AppState};

pub const ELEVATOR_SPEED: f32 = 1.0;

#[derive(Event)]
pub struct SetFloor(pub i32);

#[derive(Event)]
pub struct GoToFloor;

#[derive(Event)]
pub struct OpenDoor;

#[derive(Event)]
pub struct CloseDoor;

pub fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
}

pub fn up_down(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut component::Floor, &mut Text), With<FloorSelector>>,
    mut door_query: Query<&mut Visibility, With<component::Door>>,
    mut reader: EventReader<SetFloor>,
    mut go_reader: EventReader<GoToFloor>
    ) {
    for (mut floor, mut text) in query.iter_mut() {
        for event in reader.read() {
            if event.0 > 0 {
                commands.spawn(
                    AudioBundle{
                        source: asset_server.load("beepup.ogg"),
                        settings: PlaybackSettings{
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    });
            } else {
                commands.spawn(
                    AudioBundle{
                        source: asset_server.load("beepdown.ogg"),
                        settings: PlaybackSettings{
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    });
            }
            floor.current += event.0;
            floor.current = floor.current.clamp(0, 10);
        }
        for _ in go_reader.read() {
            let mut door_visibility = door_query.single_mut();
            *door_visibility = Visibility::Visible;
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
                commands.spawn(
                    AudioBundle{
                        source: asset_server.load("elevatormove.ogg"),
                        settings: PlaybackSettings{
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    });
            text.sections[0].value = floor_text;
            timer.duration = ELEVATOR_SPEED;
        } else if selector.current < floor.current {
            floor.current -= 1;
            let mut floor_text = floor.current.to_string();
            if floor.current == 0 {
                floor_text = "G".to_string();
            }
                commands.spawn(
                    AudioBundle{
                        source: asset_server.load("elevatormove.ogg"),
                        settings: PlaybackSettings{
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    });
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
    mut monster_query: Query<(&component::Floor, &component::Health), (With<component::Monster>, Without<component::FloorSelector>)>,
    floor_query: Query<&component::Floor, (With<component::FloorSelector>, Without<component::Monster>)>,
    mut door_query: Query<&mut Visibility, With<component::Door>>,
    asset_server: Res<AssetServer>,
    mut open_event: EventReader<OpenDoor>,
    mut close_event: EventReader<CloseDoor>,
    ) {
    let mut door_visibility = door_query.single_mut();
    let mut monster_alive = true;
    for (monster_floor, health) in monster_query.iter_mut() {
        let floor = floor_query.single();
        if monster_floor.current != floor.current {
            continue;
        }
        if health.current <= 0 {
            monster_alive = false;
            *door_visibility = Visibility::Hidden;
            app_state.set(AppState::SelectFloor);
        }
    }
    for _ in open_event.read() {
                commands.spawn(
                    AudioBundle{
                        source: asset_server.load("dooropen.ogg"),
                        settings: PlaybackSettings{
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    });
        if monster_alive {
            app_state.set(AppState::Combat);
        } else {
            app_state.set(AppState::SelectFloor);
        }
    }
    for _ in close_event.read() {
        app_state.set(AppState::SelectFloor);
    }
}

