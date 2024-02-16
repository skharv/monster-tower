use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::text::{BreakLineOn, Text2dBounds};
use rand::Rng;

use crate::component;
use crate::system::floor::{ELEVATOR_SPEED, SetFloor};

const DIRECTION_BUTTON_COLOR: Color = Color::rgb(0.5, 0.0, 0.5);
const DIRECTION_HIGHLIGHT_COLOR: Color = Color::rgb(0.75, 0.25, 0.75);
const STATS_BACKGROUND_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.7);

pub fn setup_elevator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) {
    let text = "G";
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            bottom: Val::Px(-75.0),
            right: Val::Px(-335.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::BLUE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "G".to_string(),
                            style: TextStyle {
                                font: asset_server.load("Evil-Empire.otf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        }
                    ],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::FloorSelector)
                .insert(component::Floor{ current: 0 });
        });
    });

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

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::DescriptionBox)
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            background_color: BackgroundColor(STATS_BACKGROUND_COLOR),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: text.to_string(),
                            style: TextStyle {
                                font: asset_server.load("Evil-Empire.otf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        }],
                        alignment: TextAlignment::Center,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::DescriptionText);
        });
    });

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            bottom: Val::Px(-25.0),
            right: Val::Px(-335.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(50.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(DIRECTION_BUTTON_COLOR),
            ..default()
        })
        .insert(component::UpButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "UP".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("Evil-Empire.otf"),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            }
                        ],
                        alignment: TextAlignment::Center,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    ..default()
                });
            });
    });

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            bottom: Val::Px(-120.0),
            right: Val::Px(-335.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(50.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(DIRECTION_BUTTON_COLOR),
            ..default()
        })
        .insert(component::DownButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "DOWN".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("Evil-Empire.otf"),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            }
                        ],
                        alignment: TextAlignment::Center,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    ..default()
                });
            });
    });
}

pub fn setup_player_stats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(&component::Health, &component::Armor, &component::Resistance), With<component::Player>>,
    ) {
    let (health, armor, resistances) = player_query.single();
    let stats_string = format!("Health: {}\nArmor: {}\n\nResistances:\nPhysical: {}\nMagic: {}\nFire: {}\nIce: {}\nPoison: {}\nLightning: {}\nDark: {}\nLight: {}", 
                               health.current, armor.amount, resistances.physical, resistances.magic, resistances.fire, resistances.ice, resistances.poison, resistances.lightning, resistances.dark, resistances.light);
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            background_color: BackgroundColor(STATS_BACKGROUND_COLOR),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: stats_string.to_string(),
                            style: TextStyle {
                                font: asset_server.load("Evil-Empire.otf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        }
                    ],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
        .insert(component::PlayerStatsText);
        });
    });
}

pub fn update_player_stats(
    mut text_query: Query<&mut Text, With<component::PlayerStatsText>>,
    player_query: Query<(&component::Health, &component::Armor, &component::Resistance), With<component::Player>>,
    ) {
    let (health, armor, resistances) = player_query.single();
    let mut text = text_query.single_mut();
    let stats_string = format!("Health: {}\nArmor: {}\n\nResistances:\nPhysical: {}\nMagic: {}\nFire: {}\nIce: {}\nPoison: {}\nLightning: {}\nDark: {}\nLight: {}", 
                               health.current, armor.amount, resistances.physical, resistances.magic, resistances.fire, resistances.ice, resistances.poison, resistances.lightning, resistances.dark, resistances.light);
    text.sections[0].value = stats_string.to_string();
}

pub fn navigation_buttons(
    mut up_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::UpButton>, Without<component::DownButton>)>,
    mut down_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::DownButton>, Without<component::UpButton>)>,
    mut writer: EventWriter<SetFloor>,
    mouse_button: Res<Input<MouseButton>>,
    ) {
    for (interaction, mut color) in up_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(DIRECTION_BUTTON_COLOR);
                    writer.send(SetFloor(1));
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(DIRECTION_HIGHLIGHT_COLOR);
            }
            Interaction::None => {
                *color = BackgroundColor(DIRECTION_BUTTON_COLOR);
            }
        }
    }
    for (interaction, mut color) in down_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(DIRECTION_BUTTON_COLOR);
                    writer.send(SetFloor(-1));
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(DIRECTION_HIGHLIGHT_COLOR);
            }
            Interaction::None => {
                *color = BackgroundColor(DIRECTION_BUTTON_COLOR);
            }
        }
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
