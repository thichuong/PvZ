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
    pub plant_type: PlantType,
    pub timer: Timer,
    pub health: f32,

    // Potato Mine specific
    pub armed: bool,
}

#[derive(PartialEq)]
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
#[derive(Component)]
pub struct PlantButton(pub PlantType);

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
