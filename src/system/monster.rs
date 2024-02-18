use bevy::prelude::*;
use crate::component;
use crate::system::combat::{self, Attack, DamageType, Resistances};
use crate::system::reward;
use rand::seq::SliceRandom;

#[derive(Asset, TypePath, Debug)]
pub struct Monster {
    pub name: String,
    pub sprite: String,
    pub sprite_size: Vec2,
    pub health: i32,
    pub attack: Attack,
    pub armor: i32,
    pub resistances: Resistances,
    pub description: [String; 5],
}

pub fn get_monster_details(index : i32) -> Monster {
    match index {
        0 => Monster {
            name: "Goblin".to_string(),
            sprite: "goblin.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 50,
            attack: Attack {
                damage: 3,
                damage_type: DamageType::Physical,
            },
            armor: 2,
            resistances: Resistances {
                magic: -10,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "You hear mischievous laughter echoing from behind the door, followed by the scuttling of small feet.".to_string(),
                "The door rattles as something small and nimble tries to push its way through from the other side.".to_string(),
                "A pungent odor, reminiscent of rotten eggs and stale ale, seeps through the cracks around the door.".to_string(),
                "As you approach the door, you catch glimpses of small, shadowy figures darting back and forth behind it.".to_string(),
                "You hear the unmistakable sound of chatter and the clinking of stolen treasures coming from the other side of the door.".to_string(),
            ],
        },
        1 => Monster {
            name: "Acid Sludge".to_string(),
            sprite: "acidsludge.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 60,
            attack: Attack {
                damage: 5,
                damage_type: DamageType::Poison,
            },
            armor: 5,
            resistances: Resistances {
                magic: 0,
                fire: -20,
                ice: 0,
                poison: 50,
                lightning: 0,
                dark: 0,
                light: -10,
            },
            description: [
                "A noxious odor seeps from beneath the door, accompanied by the faint sound of sizzling.".to_string(),
                "As you approach the door, you notice a viscous, bubbling liquid oozing out from underneath.".to_string(),
                "A faint, eerie glow emanates from the cracks around the door, casting strange shadows on the walls.".to_string(),
                "You hear a gurgling, bubbling noise coming from the other side, like liquid in motion.".to_string(),
                "The air feels heavy and acrid, as if the very atmosphere is being eaten away by the presence lurking behind the door.".to_string(),
            ],
        },
        2 => Monster {
            name: "Troll".to_string(),
            sprite: "troll.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 100,
            attack: Attack {
                damage: 10,
                damage_type: DamageType::Physical,
            },
            armor: 8,
            resistances: Resistances {
                magic: -10,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "You hear heavy, lumbering footsteps thudding on the ground behind the door, accompanied by the occasional scrape of wood against stone.".to_string(),
                "A deep, guttural growl echoes from the other side, followed by the unmistakable sound of wood splintering.".to_string(),
                "As you peer through the door, you catch a glimpse of a massive silhouette shifting uneasily in the dim light.".to_string(),
                "The air becomes thick with the smell of damp wood and earth, signaling the presence of something formidable on the other side.".to_string(),
                "You hear the rhythmic pounding of a large club against the ground, sending vibrations through the walls and floor.".to_string(),
            ],
        },
        3 => Monster {
            name: "Ice Snail".to_string(),
            sprite: "icesnail.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 110,
            attack: Attack {
                damage: 15,
                damage_type: DamageType::Ice,
            },
            armor: 20,
            resistances: Resistances {
                magic: 0,
                fire: -10,
                ice: 50,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "A chilly breeze seeps through the cracks around the door, carrying with it the faint scent of frost and ice.".to_string(),
                "You notice a thin layer of frost forming on the door, as if something cold and icy is pressing against it from the other side.".to_string(),
                "The air grows colder as you approach the door, and you hear the faint tinkling sound of ice crystals clinking together.".to_string(),
                "A soft, eerie glow emanates from beneath the door, casting strange shadows that dance like frost on a windowpane.".to_string(),
                "You hear a soft, rhythmic scraping sound coming from the other side, like the slow movement of icy tendrils across stone.".to_string(),
            ],
        },
        4 => Monster {
            name: "Demon".to_string(),
            sprite: "demon.png".to_string(),
            sprite_size: Vec2::new(600., 600.),
            health: 150,
            attack: Attack {
                damage: 20,
                damage_type: DamageType::Dark,
            },
            armor: 20,
            resistances: Resistances {
                magic: 0,
                fire: 50,
                ice: -10,
                poison: 0,
                lightning: 0,
                dark: 50,
                light: -10,
            },
            description: [
                "The air crackles with heat as you approach the door, and you catch a glimpse of flickering flames dancing in the darkness beyond.".to_string(),
                "A sinister laughter echoes from behind the door, accompanied by the sound of sizzling flames.".to_string(),
                "As you draw near, you feel an intense wave of heat emanating from the cracks around the door, accompanied by the scent of burning embers.".to_string(),
                "The shadows seem to writhe and dance on the walls, cast by the eerie glow of the flames seeping from beneath the door.".to_string(),
                "You hear the sound of claws scraping against stone, followed by the ominous hiss of flames as they lick at the air.".to_string(),
            ],
        },
        5 => Monster {
            name: "Ghost".to_string(),
            sprite: "ghost.png".to_string(),
            sprite_size: Vec2::new(600., 600.),
            health: 100,
            attack: Attack {
                damage: 100,
                damage_type: DamageType::Dark,
            },
            armor: 50,
            resistances: Resistances {
                magic: -10,
                fire: 0,
                ice: 0,
                poison: -10,
                lightning: -10,
                dark: 10,
                light: -10,
            },
            description: [
                "A cold chill envelops you as you approach the door, accompanied by the faint whisper of a disembodied voice.".to_string(),
                "The air shimmers with a ghostly light, casting strange, shifting shadows on the walls.".to_string(),
                "You hear the faint echo of mournful wails coming from beyond the door, sending shivers down your spine.".to_string(),
                "As you draw near, you feel a tingling sensation, as if unseen fingers are brushing against your skin.".to_string(),
                "The door rattles softly as if moved by an unseen force, accompanied by the sound of ethereal moans echoing in the distance.".to_string(),
            ],
        },
        6 => Monster {
            name: "Fire Elemental".to_string(),
            sprite: "fireelemental.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 80,
            attack: Attack {
                damage: 15,
                damage_type: DamageType::Fire,
            },
            armor: 10,
            resistances: Resistances {
                magic: 0,
                fire: 50,
                ice: -20,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "The air grows oppressively hot as you approach the door, and you catch glimpses of flickering flames dancing within.".to_string(),
                "A wave of intense heat washes over you as you draw near, accompanied by the crackling sound of roaring flames.".to_string(),
                "The door glows red-hot, emitting waves of blistering heat that distort the air around it.".to_string(),
                "As you reach out to touch the door, you feel the searing heat radiating from its surface, threatening to scorch your skin.".to_string(),
                "The smell of smoke fills your nostrils as you approach, and you hear the distant roar of flames raging on the other side.".to_string(),
            ],
        },
        7 => Monster {
            name: "Thunderstone".to_string(),
            sprite: "thunderstone.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 150,
            attack: Attack {
                damage: 20,
                damage_type: DamageType::Lightning,
            },
            armor: 50,
            resistances: Resistances {
                magic: 0,
                fire: 0,
                ice: 0,
                poison: -10,
                lightning: 20,
                dark: 0,
                light: 0,
            },
            description: [
                "As you approach, you feel the static electricity in the air building, crackling with energy.".to_string(),
                "The door vibrates with the force of contained thunder, sending shockwaves through the surrounding area.".to_string(),
                "You hear the distant roll of thunder coming from beyond the door, a precursor to the storm of power waiting to be unleashed.".to_string(),
                "The air hums with energy as you draw near, and you catch flashes of lightning dancing within the cracks around the door.".to_string(),
                "The hairs on your arms stand on end as you approach, a palpable sense of electricity filling the space around the door.".to_string(),
            ],
        },
        8 => Monster {
            name: "Light Crystals".to_string(),
            sprite: "lightcrystals.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 90,
            attack: Attack {
                damage: 17,
                damage_type: DamageType::Light,
            },
            armor: 10,
            resistances: Resistances {
                magic: 0,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: -20,
                light: 0,
            },
            description: [
                "A soft, gentle glow emanates from beneath the door, hinting at the radiance within.".to_string(),
                "As you peer through the door, the darkness seems to retreat, unable to withstand the illuminating power beyond.".to_string(),
                "The air shimmers with iridescent hues as you draw near, a celestial presence filling the corridor.".to_string(),
                "You hear the faint tinkling sound of crystalline chimes coming from beyond the door, a melodious tune echoing in the distance.".to_string(),
                "The door itself seems to radiate warmth and comfort, a testament to the soothing aura hidden within.".to_string(),
            ],
        },
        9 => Monster {
            name: "Mimic".to_string(),
            sprite: "mimic.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 70,
            attack: Attack {
                damage: 12,
                damage_type: DamageType::Physical,
            },
            armor: 5,
            resistances: Resistances {
                magic: -5,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "You peer through the door and see a chest, its wooden exterior polished and inviting.".to_string(),
                "Through the crack in the door, you catch a glimpse of what appears to be a simple wooden chest, its surface bathed in soft lamplight".to_string(),
                "A faint scent of cedar drifts through the air, hinting at the presence of a well-crafted chest beyond the door.".to_string(),
                "The faint sound of hinges creaking suggests the presence of a chest.".to_string(),
                "You sense a feeling of comfort and familiarity emanating from behind the door, as if an ordinary chest awaits your discovery.".to_string(),
            ],
        },
        10 => Monster {
            name: "Darkness".to_string(),
            sprite: "darkness.png".to_string(),
            sprite_size: Vec2::new(400., 400.),
            health: 100,
            attack: Attack {
                damage: 25,
                damage_type: DamageType::Dark,
            },
            armor: 100,
            resistances: Resistances {
                magic: 0,
                fire: 100,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "As you approach, a chill runs down your spine, and you sense a presence lurking in the darkness.".to_string(),
                "You catch the faint sound of steel scraping against stone from beyond the door.".to_string(),
                "You feel a sense of unease.".to_string(),
                "As you approach, an oppressive darkness seeps through the cracks.".to_string(),
                "The air grows colder as you draw near, and you sense a palpable sense of dread emanating from beyond.".to_string(),
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
                magic: -10,
                fire: 0,
                ice: 0,
                poison: 0,
                lightning: 0,
                dark: 0,
                light: 0,
            },
            description: [
                "You hear mischievous laughter echoing from behind the door, followed by the scuttling of small feet.".to_string(),
                "The door rattles as something small and nimble tries to push its way through from the other side.".to_string(),
                "A pungent odor, reminiscent of rotten eggs and stale ale, seeps through the cracks around the door.".to_string(),
                "As you approach the door, you catch glimpses of small, shadowy figures darting back and forth behind it.".to_string(),
                "You hear the unmistakable sound of chatter and the clinking of stolen treasures coming from the other side of the door.".to_string(),
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
    let mut index: Vec<i32> = (0..11).collect();
    let mut reward_index: Vec<i32> = (0..reward::REWARD_COUNT).collect();
    index.shuffle(&mut rng);
    reward_index.shuffle(&mut rng);
    for i in 0..11 {
        let monster = get_monster_details(index[i]);
        let texture_handle = asset_server.load(monster.sprite);
        let texture_atlas = TextureAtlas::from_grid(texture_handle, monster.sprite_size, 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let new_reward = reward::get_reward(reward_index[i]);

        let id = commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0., 0., 0.5),
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
            component::Floor { current: i as i32 },
            component::Reward {
                reward: Some(new_reward),
            },
            component::NextAction {
                action: combat::ActionType::Defend
            },
        ));
    }
}

pub fn decolor_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &component::Monster)>,
    time: Res<Time>,
    ) {
    for (mut sprite, _) in query.iter_mut() {
        if sprite.color != Color::WHITE {
            let mut sprite_color = sprite.color;
            sprite_color.set_r((sprite.color.r() + time.delta_seconds() * 2.0).min(1.0));
            sprite_color.set_g((sprite.color.g() + time.delta_seconds() * 2.0).min(1.0));
            sprite_color.set_b((sprite.color.b() + time.delta_seconds() * 2.0).min(1.0));
            sprite_color.set_a((sprite.color.a() + time.delta_seconds() * 2.0).min(1.0));
            sprite.color = sprite_color;
        }
    }
}
