use bevy::prelude::*;
use bevy::text::BreakLineOn;
use rand::Rng;

use crate::component;
use crate::system::combat::{ActionEvent, ActionType};
use crate::system::floor::{ELEVATOR_SPEED, SetFloor, GoToFloor, OpenDoor, CloseDoor};

const DIRECTION_BUTTON_COLOR: Color = Color::rgb(0.5, 0.0, 0.5);
const DIRECTION_HIGHLIGHT_COLOR: Color = Color::rgb(0.75, 0.25, 0.75);
const STATS_BACKGROUND_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.7);
const GO_BUTTON_COLOR: Color = Color::rgb(0.0, 0.75, 0.0);
const GO_HIGHLIGHT_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);

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
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::GameUi)
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

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Center,
            top: Val::Px(15.0),
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    })
        .insert(component::GameUi)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "G".to_string(),
                            style: TextStyle {
                                font: asset_server.load("Evil-Empire.otf"),
                                font_size: 80.0,
                                color: Color::WHITE,
                            },
                        }
                    ],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::Floor{ current: 0 })
            .insert(component::Timer{ duration: ELEVATOR_SPEED })
            .insert(component::FloorVisualizer);
        });

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
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                bottom: Val::Px(200.0),
                left: Val::Px(150.0),
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            border_color: BorderColor(Color::BLACK),
            ..default()
        })
        .insert(component::OpenButton)
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Open Door".to_string(),
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
            });
        });
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                bottom: Val::Px(200.0),
                right: Val::Px(150.0),
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            border_color: BorderColor(Color::BLACK),
            ..default()
        })
        .insert(component::CloseButton)
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Keep Door Closed".to_string(),
                            style: TextStyle {
                                font: asset_server.load("Evil-Empire.otf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        }],
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
            bottom: Val::Px(20.0),
            right: Val::Px(-335.0),
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::GameUi)
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
            background_color: BackgroundColor(Color::GREEN),
            ..default()
        })
        .insert(component::GoButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "GO".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("Evil-Empire.otf"),
                                    font_size: 24.0,
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
            bottom: Val::Px(-25.0),
            right: Val::Px(-335.0),
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::GameUi)
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
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::GameUi)
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
    player_query: Query<(&component::Health, &component::Mana, &component::Armor, &component::Resistance), With<component::Player>>,
    ) {
    let (health, mana, armor, resistances) = player_query.single();
    let stats_string = format!("Health: {}/{}\nMana: {}/{}\nArmor: {}\n\nResistances:\nMagic: {}\nFire: {}\nIce: {}\nPoison: {}\nLightning: {}\nDark: {}\nLight: {}", 
                               health.current, health.max, mana.current, mana.max, armor.amount, resistances.amount.magic, resistances.amount.fire, resistances.amount.ice, resistances.amount.poison, resistances.amount.lightning, resistances.amount.dark, resistances.amount.light);
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::GameUi)
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
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

pub fn setup_monster_stats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    monster_query: Query<(&component::Name, &component::Health, &component::Armor, &component::Attack, &component::Resistance, &component::Floor), With<component::Monster>>,
    floor_query: Query<&component::Floor, With<component::FloorSelector>>,
    ) {
    let current_floor = floor_query.single();
    for (name, health, armor, attack, resistances, floor) in monster_query.iter() {
        if floor.current != current_floor.current {
            continue;
        }
    let stats_string = format!("{}\nHealth: {}/{}\nArmor: {}\n\nDamage: {}\nDamageType:\n{}\n\nResistances:\nMagic: {}\nFire: {}\nIce: {}\nPoison: {}\nLightning: {}\nDark: {}\nLight: {}", 
                               name.name, health.current, health.max, armor.amount, attack.damage, attack.damage_type, resistances.amount.magic, resistances.amount.fire, resistances.amount.ice, resistances.amount.poison, resistances.amount.lightning, resistances.amount.dark, resistances.amount.light);
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::End,
            margin: UiRect::axes(Val::Px(-5.), Val::Px(5.0)),
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::InspectUi)
    .insert(component::CombatUi)
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
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
            .insert(component::MonsterStatsText);
        });
    });
    }
}

pub fn setup_combat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) {
    let font_handle = asset_server.load("Evil-Empire.otf");
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            bottom: Val::Px(5.0),
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    }).insert(component::CombatUi)
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .insert(component::AttackButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "ATTACK".to_string(),
                            style: TextStyle {
                                font: font_handle.clone(),
                                font_size: 18.0,
                                color: Color::BLACK,
                            },
                        }
                    ],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::AttackButtonText);
        });
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::RED),
            ..default()
        })
        .insert(component::FireballButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "FIREBALL".to_string(),
                            style: TextStyle {
                                font: font_handle.clone(),
                                font_size: 18.0,
                                color: Color::WHITE,
                            },
                        }
                    ],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::FireballButtonText);
        });
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::BLUE),
            ..default()
        })
        .insert(component::IceSpearButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "ICE SPEAR".to_string(),
                            style: TextStyle {
                                font: font_handle.clone(),
                                font_size: 18.0,
                                color: Color::WHITE,
                            },
                        }
                    ],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::IceSpearButtonText);
        });
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::YELLOW),
            ..default()
        })
        .insert(component::ShockButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "SHOCK".to_string(),
                            style: TextStyle {
                                font: font_handle.clone(),
                                font_size: 18.0,
                                color: Color::BLACK,
                            },
                        }
                    ],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::ShockButtonText);
        });
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::GRAY),
            ..default()
        })
        .insert(component::BlockButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Block".to_string(),
                            style: TextStyle {
                                font: font_handle.clone(),
                                font_size: 18.0,
                                color: Color::WHITE,
                            },
                        }
                    ],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::BlockButtonText);
        });
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::GREEN),
            ..default()
        })
        .insert(component::InspectButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Inspect".to_string(),
                            style: TextStyle {
                                font: font_handle.clone(),
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

pub fn open_door_buttons(
    mut open_button_query: Query<(&mut Interaction, &mut BackgroundColor), (With<component::OpenButton>, Without<component::CloseButton>)>,
    mut close_button_query: Query<(&mut Interaction, &mut BackgroundColor), (With<component::CloseButton>, Without<component::OpenButton>)>,
    mouse_button: Res<Input<MouseButton>>,
    mut open_event: EventWriter<OpenDoor>,
    mut close_event: EventWriter<CloseDoor>,
    ) {
    for (interaction, mut color) in open_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(Color::DARK_GRAY);
                    open_event.send(OpenDoor);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
        }
    }
    for (interaction, mut color) in close_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(Color::DARK_GRAY);
                    close_event.send(CloseDoor);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
        }
    }
}

pub fn update_combat(
    player_query: Query<(&component::Attack, &component::Fireball, &component::IceSpear, &component::Shock), With<component::Player>>,
    mut attack_button_query: Query<&mut Text, (With<component::AttackButtonText>, Without<component::FireballButtonText>, Without<component::IceSpearButtonText>, Without<component::ShockButtonText>)>,
    mut fireball_button_query: Query<&mut Text, (With<component::FireballButtonText>, Without<component::AttackButtonText>, Without<component::IceSpearButtonText>, Without<component::ShockButtonText>)>,
    mut ice_spear_button_query: Query<&mut Text, (With<component::IceSpearButtonText>, Without<component::AttackButtonText>, Without<component::FireballButtonText>, Without<component::ShockButtonText>)>,
    mut shock_button_query: Query<&mut Text, (With<component::ShockButtonText>, Without<component::AttackButtonText>, Without<component::FireballButtonText>, Without<component::IceSpearButtonText>)>,
    ) {
    let (attack, fireball, ice_spear, shock) = player_query.single();
    let mut attack_button = attack_button_query.single_mut();
    let mut fireball_button = fireball_button_query.single_mut();
    let mut ice_spear_button = ice_spear_button_query.single_mut();
    let mut shock_button = shock_button_query.single_mut();
    attack_button.sections[0].value = format!("ATTACK\n{} damage: {}", attack.damage_type, attack.damage);
    fireball_button.sections[0].value = format!("FIREBALL\nMana Cost: {}\nDamage: {}", fireball.mana_cost, fireball.damage);
    ice_spear_button.sections[0].value = format!("ICE SPEAR\nMana Cost: {}\nDamage: {}", ice_spear.mana_cost, ice_spear.damage);
    shock_button.sections[0].value = format!("SHOCK\nMana Cost: {}\nDamage: {}", shock.mana_cost, shock.damage);
}

pub fn show_combat(
    mut box_query: Query<&mut Visibility, (With<component::CombatUi>, Without<component::InspectUi>)>,
    ) {
    for mut box_visibility in box_query.iter_mut() {
        *box_visibility = Visibility::Visible;
    }
}

pub fn hide_combat(
    mut box_query: Query<&mut Visibility, With<component::CombatUi>>,
    ) {
    for mut box_visibility in box_query.iter_mut() {
        *box_visibility = Visibility::Hidden;
    }
}

pub fn show_monster_stats(
    monster_query: Query<(&component::Floor, &component::Health), (With<component::Inspected>, With<component::Monster>, Without<component::FloorSelector>)>,
    floor_query: Query<&component::Floor, (With<component::FloorSelector>, Without<component::Monster>)>,
    mut box_query: Query<&mut Visibility, With<component::InspectUi>>,
    ) {
    let mut box_visibility = box_query.single_mut();
    if *box_visibility == Visibility::Visible {
        return;
    }
    for (monster_floor, monster_health) in monster_query.iter() {
        for floor in floor_query.iter() {
            if monster_floor.current != floor.current {
                continue;
            }
            if monster_health.current <= 0 {
                continue;
            }
            *box_visibility = Visibility::Visible;
        }
    } 
}

pub fn setup_monster_actions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            bottom: Val::Px(250.0),
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(component::CombatUi)
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
                                value: "Next Action:".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("Evil-Empire.otf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            }],
                            alignment: TextAlignment::Center,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    ..default()
                });
            });
        });
    let mut texture_handle = asset_server.load("attackicon.png");
    let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 1, 1, None, None);
    let mut texture_atlas_handle = texture_atlases.add(texture_atlas);
    let position = Vec3::new(0.0, 200.0, 5.0);

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: position,
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    },
    component::PhysicalIcon,
    component::CombatUi,
    component::Icon));

    texture_handle = asset_server.load("waiticon.png");
    texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 1, 1, None, None);
    texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: position,
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    },
    component::WaitIcon,
    component::CombatUi,
    component::Icon));

    texture_handle = asset_server.load("defendicon.png");
    texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 1, 1, None, None);
    texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: position,
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
    },
    component::DefendIcon,
    component::CombatUi,
    component::Icon));
}

pub fn update_player_stats(
    mut text_query: Query<&mut Text, With<component::PlayerStatsText>>,
    player_query: Query<(&component::Health, &component::Mana, &component::Armor, &component::Resistance), With<component::Player>>,
    ) {
    let (health, mana, armor, resistances) = player_query.single();
    let mut text = text_query.single_mut();
    let stats_string = format!("Health: {}/{}\nMana: {}/{}\nArmor: {}\n\nResistances:\nMagic: {}\nFire: {}\nIce: {}\nPoison: {}\nLightning: {}\nDark: {}\nLight: {}", 
                               health.current, health.max, mana.current, mana.max, armor.amount, resistances.amount.magic, resistances.amount.fire, resistances.amount.ice, resistances.amount.poison, resistances.amount.lightning, resistances.amount.dark, resistances.amount.light);
    text.sections[0].value = stats_string.to_string();
}

pub fn update_monster_stats(
    mut text_query: Query<&mut Text, With<component::MonsterStatsText>>,
    monster_query: Query<(&component::Name, &component::Health, &component::Armor, &component::Attack, &component::Resistance, &component::Floor), (With<component::Inspected>, With<component::Monster>, Without<component::FloorSelector>)>,
    floor_query: Query<&component::Floor, With<component::FloorSelector>>,
    ) {
    let current_floor = floor_query.single();
    for (name, health, armor, attack, resistances, floor) in monster_query.iter() {
        if floor.current != current_floor.current {
            continue;
        }
        let mut text = text_query.single_mut();
    let stats_string = format!("{}\nHealth: {}/{}\nArmor: {}\n\nDamage: {}\nDamageType:\n{}\n\nResistances:\nMagic: {}\nFire: {}\nIce: {}\nPoison: {}\nLightning: {}\nDark: {}\nLight: {}", 
                               name.name, health.current, health.max, armor.amount, attack.damage, attack.damage_type, resistances.amount.magic, resistances.amount.fire, resistances.amount.ice, resistances.amount.poison, resistances.amount.lightning, resistances.amount.dark, resistances.amount.light);
        text.sections[0].value = stats_string.to_string();
    }
}

pub fn navigation_buttons(
    mut up_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::UpButton>, Without<component::DownButton>, Without<component::GoButton>)>,
    mut down_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::DownButton>, Without<component::UpButton>, Without<component::GoButton>)>,
    mut go_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::GoButton>, Without<component::DownButton>, Without<component::UpButton>)>,
    mut up_down_writer: EventWriter<SetFloor>,
    mut go_writer: EventWriter<GoToFloor>,
    mouse_button: Res<Input<MouseButton>>,
    ) {
    for (interaction, mut color) in up_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(DIRECTION_BUTTON_COLOR);
                    up_down_writer.send(SetFloor(1));
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
                    up_down_writer.send(SetFloor(-1));
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
    for (interaction, mut color) in go_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(GO_BUTTON_COLOR);
                    go_writer.send(GoToFloor);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(GO_HIGHLIGHT_COLOR);
            }
            Interaction::None => {
                *color = BackgroundColor(GO_BUTTON_COLOR);
            }
        }
    }
}

pub fn show_reward(
    mut box_query: Query<&mut Visibility, With<component::RewardUi>>,
    ) {
    for mut box_visibility in box_query.iter_mut() {
        *box_visibility = Visibility::Visible;
    }
}

pub fn hide_reward(
    mut box_query: Query<&mut Visibility, With<component::RewardUi>>,
    ) {
    for mut box_visibility in box_query.iter_mut() {
        *box_visibility = Visibility::Hidden;
    }
}

pub fn setup_reward(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) {
    let font_handle = asset_server.load("Evil-Empire.otf");
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
    .insert(component::RewardUi)
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(STATS_BACKGROUND_COLOR),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "You found a reward!".to_string(),
                            style: TextStyle {
                                font: font_handle.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        }],
                        alignment: TextAlignment::Center,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                },
                ..default()
            })
            .insert(component::RewardText);
        });
    });
}

pub fn combat_buttons(
    mut attack_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::AttackButton>, Without<component::FireballButton>, Without<component::IceSpearButton>, Without<component::ShockButton>, Without<component::BlockButton>, Without<component::InspectButton>)>,
    mut fireball_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::FireballButton>, Without<component::AttackButton>, Without<component::IceSpearButton>, Without<component::ShockButton>, Without<component::BlockButton>, Without<component::InspectButton>)>,
    mut ice_spear_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::IceSpearButton>, Without<component::FireballButton>, Without<component::AttackButton>, Without<component::ShockButton>, Without<component::BlockButton>, Without<component::InspectButton>)>,
    mut shock_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::ShockButton>, Without<component::FireballButton>, Without<component::IceSpearButton>, Without<component::AttackButton>, Without<component::BlockButton>, Without<component::InspectButton>)>,
    mut block_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::BlockButton>, Without<component::FireballButton>, Without<component::IceSpearButton>, Without<component::ShockButton>, Without<component::AttackButton>, Without<component::InspectButton>)>,
    mut inspect_button_query: Query<(&Interaction, &mut BackgroundColor), (With<component::InspectButton>, Without<component::FireballButton>, Without<component::IceSpearButton>, Without<component::ShockButton>, Without<component::BlockButton>, Without<component::AttackButton>)>,
    mut writer: EventWriter<ActionEvent>,
    mouse_button: Res<Input<MouseButton>>,
    ) {
    for (interaction, mut color) in attack_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(Color::WHITE);
                    writer.send(ActionEvent(ActionType::Attack));
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::WHITE);
            }
        }
    }
    for (interaction, mut color) in fireball_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(Color::RED);
                    writer.send(ActionEvent(ActionType::Fireball));
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::RED);
            }
        }
    }
    for (interaction, mut color) in ice_spear_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(Color::BLUE);
                    writer.send(ActionEvent(ActionType::IceSpear));
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::BLUE);
            }
        }
    }
    for (interaction, mut color) in shock_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(Color::YELLOW);
                    writer.send(ActionEvent(ActionType::Shock));
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::YELLOW);
            }
        }
    }
    for (interaction, mut color) in block_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(Color::BLACK);
                    writer.send(ActionEvent(ActionType::Defend));
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::BLACK);
            }
        }
    }
    for (interaction, mut color) in inspect_button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    *color = BackgroundColor(Color::GREEN);
                    writer.send(ActionEvent(ActionType::Inspect));
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::DARK_GREEN);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::GREEN);
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
        text.sections[0].value = monster_description.descriptions[rng.gen_range(0..5)].clone();
    }
}


pub fn hide_description(
    mut box_query: Query<&mut Visibility, With<component::DescriptionBox>>,
    ) {
    let mut box_visibility = box_query.single_mut();
    *box_visibility = Visibility::Hidden;
}
