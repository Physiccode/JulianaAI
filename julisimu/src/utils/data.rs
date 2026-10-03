use bevy::prelude::*;

pub const CAMERA_SENSIVITY:f32 = 0.003;
pub const CAMERA_SPEED:f32 = 9.0;
pub const JULIANA_SPEED:f32 = 5.0;
pub const EAT_DISTANCE:f32 = 0.6;
pub const MAX_FOOD:u8 = 2;
pub const HALF_SIZE = 19;

#[derive(Component)]
pub struct Juliana;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct MovableCamera;

