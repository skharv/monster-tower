use bevy::prelude::*;
use crate::component;

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
pub struct Monsters {
    pub monsters: Vec<Monster>
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
pub struct Monster {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub armor: i32,
}

#[derive(Resource, Debug)]
pub struct MonsterHandle(Handle<Monsters>);

pub fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) {
    let monsters = MonsterHandle(asset_server.load("monsters.yaml"));
    commands.insert_resource(monsters);
    commands.spawn(Camera2dBundle::default());
}

pub fn generate(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    monster: Res<MonsterHandle>,
    mut monsters: ResMut<Assets<Monsters>>,
    ) {
    warn!("{:?}", monster);
    if let Some(monster) = monsters.remove(monster.0.id()) {
        warn!("{:?}", monster);
        for m in monster.monsters {
            warn!("{:?}", m);
        }
    }
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
            }), component::Floor{ current: 0 }
            ));
}
