use bevy::prelude::*;
use crate::component;
use crate::system::combat::{DamageType, Attack, Resistances};
use crate::system::reward;
use rand::Rng;

#[derive(Asset, TypePath, Debug)]
pub struct Monster {
    pub name: String,
    pub sprite: String,
    pub sprite_size: Vec2,
    pub health: i32,
    pub attack: Attack,
    pub armor: i32,
    pub resistances: Resistances,
    pub description: [String; 3],
}

pub fn get_monster_details(index : i32) -> Monster {
    match index {
        0 => Monster {
            name: "Goblin".to_string(),
            sprite: "goblin.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 10,
            attack: Attack {
                damage: 5,
                damage_type: DamageType::Physical,
            },
            armor: 2,
            resistances: Resistances {
                physical: -5,
                magic: -10,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "A small, green creature with a big nose and pointy ears.".to_string(),
                "It is wearing a dirty, brown tunic and carrying a small, wooden club.".to_string(),
                "It looks at you with a mix of fear and anger.".to_string(),
            ],
        },
        1 => Monster {
            name: "Orc".to_string(),
            sprite: "orc.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 20,
            attack: Attack {
                damage: 10,
                damage_type: DamageType::Physical,
            },
            armor: 5,
            resistances: Resistances {
                physical: 0,
                magic: 0,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "A large, green creature with a big nose and pointy ears.".to_string(),
                "It is wearing a dirty, brown tunic and carrying a large, wooden club.".to_string(),
                "It looks at you with a mix of fear and anger.".to_string(),
            ],
        },
        2 => Monster {
            name: "Troll".to_string(),
            sprite: "troll.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 30,
            attack: Attack {
                damage: 15,
                damage_type: DamageType::Physical,
            },
            armor: 10,
            resistances: Resistances {
                physical: 0,
                magic: 0,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "A huge, green creature with a big nose and pointy ears.".to_string(),
                "It is wearing a dirty, brown tunic and carrying a large, wooden club.".to_string(),
                "It looks at you with a mix of fear and anger.".to_string(),
            ],
        },
        3 => Monster {
            name: "Dragon".to_string(),
            sprite: "dragon.png".to_string(),
            sprite_size: Vec2::new(600., 600.),
            health: 50,
            attack: Attack {
                damage: 25,
                damage_type: DamageType::Fire,
            },
            armor: 15,
            resistances: Resistances {
                physical: 0,
                magic: 0,
                fire: 50,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "A huge, green creature with a big nose and pointy ears.".to_string(),
                "It is wearing a dirty, brown tunic and carrying a large, wooden club.".to_string(),
                "It looks at you with a mix of fear and anger.".to_string(),
            ],
        },
        4 => Monster {
            name: "Demon".to_string(),
            sprite: "demon.png".to_string(),
            sprite_size: Vec2::new(600., 600.),
            health: 100,
            attack: Attack {
                damage: 50,
                damage_type: DamageType::Dark,
            },
            armor: 20,
            resistances: Resistances {
                physical: 0,
                magic: 0,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 50,
                light: 0,
            },
            description: [
                "A huge, green creature with a big nose and pointy ears.".to_string(),
                "It is wearing a dirty, brown tunic and carrying a large, wooden club.".to_string(),
                "It looks at you with a mix of fear and anger.".to_string(),
            ],
        },
        5 => Monster {
            name: "Devil".to_string(),
            sprite: "devil.png".to_string(),
            sprite_size: Vec2::new(600., 600.),
            health: 200,
            attack: Attack {
                damage: 100,
                damage_type: DamageType::Dark,
            },
            armor: 50,
            resistances: Resistances {
                physical: 0,
                magic: 0,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 100,
                light: 0,
            },
            description: [
                "A huge, green creature with a big nose and pointy ears.".to_string(),
                "It is wearing a dirty, brown tunic and carrying a large, wooden club.".to_string(),
                "It looks at you with a mix of fear and anger.".to_string(),
            ],
        },
        6 => Monster {
            name: "FireElemental".to_string(),
            sprite: "fireelemental.png".to_string(),
            sprite_size: Vec2::new(600., 600.),
            health: 500,
            attack: Attack {
                damage: 250,
                damage_type: DamageType::Fire,
            },
            armor: 100,
            resistances: Resistances {
                physical: 0,
                magic: 0,
                fire: 100,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "A huge, green creature with a big nose and pointy ears.".to_string(),
                "It is wearing a dirty, brown tunic and carrying a large, wooden club.".to_string(),
                "It looks at you with a mix of fear and anger.".to_string(),
            ],
        },
        _ => Monster {
            name: "Goblin".to_string(),
            sprite: "goblin.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 10,
            attack: Attack {
                damage: 5,
                damage_type: DamageType::Physical,
            },
            armor: 2,
            resistances: Resistances {
                physical: -5,
                magic: -10,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "A small, green creature with a big nose and pointy ears.".to_string(),
                "It is wearing a dirty, brown tunic and carrying a small, wooden club.".to_string(),
                "It looks at you with a mix of fear and anger.".to_string(),
            ],
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
    let mut rng = rand::thread_rng();
    for i in 0..7 {
        let monster = get_monster_details(i);
        info!("Loading {:?}",monster.sprite);
        let texture_handle = asset_server.load(monster.sprite);
        let texture_atlas = TextureAtlas::from_grid(texture_handle, monster.sprite_size, 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let reward_index = rng.gen_range(0..reward::REWARD_COUNT);
        let reward = reward::get_reward(reward_index);

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
            component::Attack {
                damage: monster.attack.damage,
                damage_type: monster.attack.damage_type,
            },
            component::Armor { amount: monster.armor },
            component::Resistance { amount: monster.resistances },
            component::Description {
                descriptions: monster.description,
            },
            component::Floor { current: i },
            component::Reward {
                name: reward.name,
                physical_resistance: reward.physical_resistance,
                magic_resistance: reward.magic_resistance,
                fire_resistance: reward.fire_resistance,
                ice_resistance: reward.ice_resistance,
                poison_resistance: reward.poison_resistance,
                lightning_resistance: reward.lightning_resistance,
                dark_resistance: reward.dark_resistance,
                light_resistance: reward.light_resistance,
                health: reward.health,
                damage: reward.damage,
                armor: reward.armor,
                damage_type: reward.damage_type,
            },
        ));
    }
}

