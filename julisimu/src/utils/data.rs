use bevy::prelude::*;

pub const CAMERA_SENSIVITY:f32 = 0.003;
pub const CAMERA_SPEED:f32 = 9.0;

pub const JULIANA_SPEED:f32 = 6.0;
// JDE = JULIANA DEFAULT ENERGY
pub const JDE:f32 = 90.0;
// JME = JULIANA MAX ENERGY
pub const JME:f32 = 300.0;

pub const EAT_DISTANCE:f32 = 0.6;
pub const MAX_FOOD:usize = 3;
// I'll see hwat happens if i have f32,then ill maybe change this to i32
pub const HALF_SIZE:f32 = 19.0;
pub const STARVATION_RATE:f32 = 5.0; // 5 hp/frame
                                      
// AJDE = Anti-Juliana Default Energy
pub const AJDE:f32 = 150.0;
pub const AJME:f32 = 200.0;
// AJSR = Anti-Juliana Starvation Rate
// Make them nearly impossible to die unless juliana manually kills them
pub const AJSR:f32 = 1.0;
// AJSR = Anti-Juliana Damage rate
pub const AJDR:f32 = 250.0;
// AJS = Anti-Juliana speed;
pub const AJS:f32 = 5.0;

#[derive(Component)]
pub struct Juliana{
    pub speed:f32,
    pub current_energy:f32,
    pub max_energy:f32,
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct MovableCamera;

#[derive(Component)]
pub struct AntiJuliana {
    pub current_energy:f32,
    pub max_energy:f32,
    pub speed:f32,
    pub attack_power:f32,
}

