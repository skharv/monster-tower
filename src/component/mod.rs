use bevy::prelude::*;
use crate::system::{combat, reward};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Title;

#[derive(Component)]
pub struct PlayerStatsText;

#[derive(Component)]
pub struct GameUi;

#[derive(Component)]
pub struct Win;

#[derive(Component)]
pub struct MonsterStatsText;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Door;

#[derive(Component)]
pub struct CombatUi;

#[derive(Component)]
pub struct RewardUi;

#[derive(Component)]
pub struct RewardText;

#[derive(Component)]
pub struct PhysicalIcon;

#[derive(Component)]
pub struct WaitIcon;

#[derive(Component)]
pub struct DefendIcon;

#[derive(Component)]
pub struct Icon;

#[derive(Component)]
pub struct AttackButton;

#[derive(Component)]
pub struct AttackButtonText;

#[derive(Component)]
pub struct FireballButton;

#[derive(Component)]
pub struct FireballButtonText;

#[derive(Component)]
pub struct IceSpearButton;

#[derive(Component)]
pub struct IceSpearButtonText;

#[derive(Component)]
pub struct ShockButton;

#[derive(Component)]
pub struct ShockButtonText;

#[derive(Component)]
pub struct BlockButton;

#[derive(Component)]
pub struct OpenButton;

#[derive(Component)]
pub struct CloseButton;

#[derive(Component)]
pub struct Loss;

#[derive(Component)]
pub struct BlockButtonText;

#[derive(Component)]
pub struct InspectButton;

#[derive(Component)]
pub struct InspectUi;

#[derive(Component)]
pub struct Inspected;

#[derive(Component)]
pub struct GoButton;

#[derive(Component)]
pub struct UpButton;

#[derive(Component)]
pub struct DownButton;

#[derive(Component)]
pub struct FloorSelector;

#[derive(Component)]
pub struct FloorVisualizer;

#[derive(Component)]
pub struct Floor {
    pub current: i32,
}

#[derive(Component)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}

#[derive(Component)]
pub struct Mana {
    pub max: i32,
    pub current: i32,
}

#[derive(Component)]
pub struct Attack {
    pub damage: i32,
    pub damage_type: combat::DamageType,
}

#[derive(Component)]
pub struct Fireball {
    pub damage: i32,
    pub damage_type: combat::DamageType,
    pub mana_cost: i32,
}

#[derive(Component)]
pub struct IceSpear {
    pub damage: i32,
    pub damage_type: combat::DamageType,
    pub mana_cost: i32,
}

#[derive(Component)]
pub struct Shock {
    pub damage: i32,
    pub damage_type: combat::DamageType,
    pub mana_cost: i32,
}

#[derive(Component)]
pub struct Resistance {
    pub amount: combat::Resistances
}

#[derive(Component)]
pub struct Reward {
    pub reward: Option<reward::Reward>,
}

#[derive(Component)]
pub struct Description {
    pub descriptions: [String; 5],
}

#[derive(Component)]
pub struct DescriptionText;

#[derive(Component)]
pub struct DescriptionBox;

#[derive(Component)]
pub struct Armor {
    pub amount: i32,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Timer {
    pub duration: f32,
}

#[derive(Component)]
pub struct NextAction {
    pub action: combat::ActionType,
}
