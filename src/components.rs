use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlantType {
    Peashooter,
    Sunflower,
    WallNut,
    PotatoMine,
}

#[derive(Component)]
pub struct Plant {
    pub kind: PlantType,
    pub timer: Timer,
    pub health: f32,

    // Potato Mine specific
    pub armed: bool,
}

#[derive(PartialEq, Eq)]
pub enum ZombieState {
    Walking,
    Eating(Entity), // Entity being eaten
}

#[derive(Component)]
pub struct Zombie {
    pub health: f32,
    pub state: ZombieState,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct GridCell {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct SunText;

// Marked for buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Plant(PlantType),
    Shovel,
}

// Marked for buttons
#[derive(Component)]
pub struct ToolButton(pub Tool);

#[derive(Component)]
pub struct Explosion {
    pub timer: Timer,
    pub radius: f32,
    pub damage: f32,
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub timer: Timer,
}

#[derive(Component)]
pub struct Cursor;
