use bevy::{prelude::*, render::view::visibility};
use crate::component;

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
pub struct Monster {
    pub name: String,
    pub sprite: String,
    pub health: i32,
    pub attack: i32,
    pub armor: i32,
}

pub fn get_monster_details(index : i32) -> Monster {
    match index {
        0 => Monster {
            name: "Goblin".to_string(),
            sprite: "goblin.png".to_string(),
            health: 10,
            attack: 5,
            armor: 2,
        },
        1 => Monster {
            name: "Orc".to_string(),
            sprite: "orc.png".to_string(),
            health: 20,
            attack: 10,
            armor: 5,
        },
        2 => Monster {
            name: "Troll".to_string(),
            sprite: "troll.png".to_string(),
            health: 30,
            attack: 15,
            armor: 10,
        },
        3 => Monster {
            name: "Dragon".to_string(),
            sprite: "dragon.png".to_string(),
            health: 50,
            attack: 20,
            armor: 15,
        },
        4 => Monster {
            name: "Demon".to_string(),
            sprite: "demon.png".to_string(),
            health: 100,
            attack: 50,
            armor: 20,
        },
        5 => Monster {
            name: "Devil".to_string(),
            sprite: "devil.png".to_string(),
            health: 200,
            attack: 100,
            armor: 50,
        },
        6 => Monster {
            name: "FireElemental".to_string(),
            sprite: "fireelemental.png".to_string(),
            health: 500,
            attack: 200,
            armor: 100,
        },
        _ => Monster {
            name: "Goblin".to_string(),
            sprite: "goblin.png".to_string(),
            health: 10,
            attack: 5,
            armor: 2,
        },
    }
}

pub fn load(
    mut commands: Commands,
    ) {
    commands.spawn(Camera2dBundle::default());
}

pub fn generate(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    ) {
    for i in 0..7 {
        let monster = get_monster_details(i);
        info!("Loading {:?}",monster.sprite);
        let texture_handle = asset_server.load(monster.sprite);
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(256., 256.), 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let id = commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0., 0., 1.),
            visibility: Visibility::Hidden,
            ..default()
            }).id();
        
        commands.entity(id).insert((
            component::Monster,
            component::Name { name: monster.name },
            component::Health { current: monster.health, max: monster.health },
            component::Attack { damage: monster.attack },
            component::Armor { amount: monster.armor },
            component::Floor { current: i },
        ));
    }
}

