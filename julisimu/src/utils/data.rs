use bevy::prelude::*;
use rand::prelude::*;

pub const CAMERA_SENSIVITY:f32 = 0.003;
pub const CAMERA_SPEED:f32 = 9.0;

pub const JULIANA_SPEED:f32 = 8.0;
// JDE = JULIANA DEFAULT ENERGY
pub const JDE:f32 = 90.0;
// JME = JULIANA MAX ENERGY
pub const JME:f32 = 300.0;

pub const EAT_DISTANCE:f32 = 0.6;
pub const MAX_FOOD:usize = 20;
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
    pub eaten_food:u8,
}
impl Default for Juliana {
    pub default()->Self {
        Self {
            speed:JULIANA_SPEED,
            current_energy:JDE,
            max_energy:JME,
            eaten_food:0,
        }
    }
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
impl Default for AntiJuliana {
    pub default()->Self {
        Self{
            current_energy:AJDE,
            max_energy:AJME,
            speed:AJS,
            attack_power:AJDR,
        }
    }
}

#[derive(Resource,Component,Clone)]
pub struct Brain{
    /*Weights:
     *0:How much i care about food x 
    1:How much i care about food z 
    2:How much i care about predator's x
    3:How much i care about predator z
     * */
    pub weights:[f32;4],
}

impl Default for Brain {
    fn default()->Self {
        //Random brain for the first ancestor
        let mut rng = rand::rng();
        Self {
            weights: [
                rng.random_range(-1.0..=1.0), // Food's x preference
                rng.random_range(-1.0..=1.0), // Food's z preference
                rng.random_range(-1.0..=1.0), // Predator's x avoidance
                rng.random_range(-1.0..=1.0), // Predator's z avoidance
            ],
            sucess_unit:0,
        }
    }
}

#[derive(Resource,Default)]
pub struct BestSucess {
    best_sucess:u8,
}
