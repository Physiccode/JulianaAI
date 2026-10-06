use bevy::prelude::*;

pub const CAMERA_SENSIVITY:f32 = 0.003;
pub const CAMERA_SPEED:f32 = 9.0;
pub const JULIANA_SPEED:f32 = 6.0;
pub const EAT_DISTANCE:f32 = 0.6;
pub const MAX_FOOD:usize = 2;
// I'll see hwat happens if i have f32,then ill maybe change this to i32
pub const HALF_SIZE:f32 = 19.0;

#[derive(Component)]
pub struct Juliana;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct MovableCamera;

