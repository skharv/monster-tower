use bevy::prelude::*;

use crate::component;
use crate::system::combat;
use crate::AppState;

pub fn spawn(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    ) {
    commands.spawn((
            component::Player,
            component::Health {
                max: 100,
                current: 100,
            },
            component::Mana {
                max: 100,
                current: 100,
            },
            component::Attack {
                damage: 10,
                damage_type: combat::DamageType::Physical,
            },
            component::Fireball {
                damage: 20,
                damage_type: combat::DamageType::Fire,
                mana_cost: 5,
            },
            component::IceSpear {
                damage: 20,
                damage_type: combat::DamageType::Ice,
                mana_cost: 5,
            },
            component::Shock {
                damage: 20,
                damage_type: combat::DamageType::Lightning,
                mana_cost: 5,
            },
            component::Armor {
                amount: 5,
            },
            component::Resistance {
                amount: combat::Resistances {
                    magic: 0,
                    fire: 0,
                    ice: 0,
                    poison: 0,
                    lightning: 0,
                    dark: 0,
                    light: 0,
                },
            }));

    let mut texture_handle = asset_server.load("defeat.png");
    let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(800., 800.), 1, 1, None, None);
    let mut texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0., 0., 10.),
        visibility: Visibility::Hidden,
        ..Default::default()
    },
    component::Loss));

    texture_handle = asset_server.load("title.png");
    texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(800., 800.), 1, 1, None, None);
    texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0., 0., 10.),
        ..Default::default()
    },
    component::Title));

    texture_handle = asset_server.load("victory.png");
    texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(800., 800.), 1, 1, None, None);
    texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0., 0., 10.),
        visibility: Visibility::Hidden,
        ..Default::default()
    },
    component::Win));
}

pub fn hide_title_screen(
    mut commands: Commands,
    title_query: Query<Entity, (With<component::Title>, Without<Node>)>,
    mut ui_query: Query<&mut Visibility, (With<component::GameUi>, Without<component::Title>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mouse_button: Res<Input<MouseButton>>,
    ) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let title = title_query.single();
        commands.entity(title).despawn();
        for mut ui_visibility in ui_query.iter_mut() {
            *ui_visibility = Visibility::Visible;
        }
        app_state.set(AppState::SelectFloor);
    }
}

pub fn hide_ui_show_loss(
    mut loss_query: Query<&mut Visibility, (With<component::Loss>, Without<Node>)>,
    mut ui_query: Query<&mut Visibility, (With<Node>, Without<component::Loss>)>,
    ) {
    for mut loss_visibility in loss_query.iter_mut() {
        *loss_visibility = Visibility::Visible;
    }
    for mut ui_visibility in ui_query.iter_mut() {
        *ui_visibility = Visibility::Hidden;
    }
}

pub fn hide_ui_show_victory(
    mut win_query: Query<&mut Visibility, (With<component::Win>, Without<Node>)>,
    mut ui_query: Query<&mut Visibility, (With<Node>, Without<component::Win>)>,
    ) {
    for mut win_visibility in win_query.iter_mut() {
        *win_visibility = Visibility::Visible;
    }
    for mut ui_visibility in ui_query.iter_mut() {
        *ui_visibility = Visibility::Hidden;
    }
}
