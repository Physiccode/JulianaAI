use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode,CursorOptions};
use super::data::*;
use bevy::post_process::bloom::Bloom;
use rand::prelude::*;

pub fn setup(
    mut commands:Commands,
    asset_server:Res<AssetServer>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    mut meshes:ResMut<Assets<Mesh>>
    ) {

     //Spawn camera
   commands.spawn((
           Camera3d::default(),
           Transform::from_xyz(0.0,2.0,4.0)
            .looking_at(Vec3::ZERO,Vec3::Y)
           ,
           Camera::default(),
           Bloom::NATURAL,
           MovableCamera
           ));

     
    // Spawn first juliana
    commands.spawn((
        WorldAssetRoot(asset_server.load("bacteria.glb#Scene0")),
        Transform::from_xyz(0.0,0.5,0.0).with_scale(Vec3::splat(0.01)),
        Juliana::default(),
        Brain::default(),
    ));

    let (x,z) = get_correct_pos();
    // Spawn first predator 
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial{
            base_color:Color::from(bevy::color::palettes::tailwind::FUCHSIA_950),
            unlit:false,
            ..default()
        })),
        Transform::from_xyz(x,1.5,z),
        AntiJuliana::default(),
    ));
}

pub fn move_camera(
    mut cursor_options:Single<&mut CursorOptions>,
    accumulated_mouse_motion:Res<AccumulatedMouseMotion>,
    time:Res<Time>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut query:Query<&mut Transform,With<MovableCamera>>

    ) {

    let mut transform = match query.single_mut(){
        Ok(t)=>t,
        Err(_)=>return,
    };

    // Mouse motion 
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Prevent Gimbal lock 
        let (mut yaw,mut pitch,_) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw -= delta.x * CAMERA_SENSIVITY;
        pitch -= delta.y * CAMERA_SENSIVITY;

        // Clamp pitch 
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.05; // Don't allow the player to look
                                                            // exactly 90º up or down
        pitch = pitch.clamp(-max_pitch,max_pitch);
        // Force roll to stay exactly 0.0
        transform.rotation = Quat::from_euler(EulerRot::YXZ,yaw,pitch,0.0);
    }


    // Keyboard motion
    let move_direction = CAMERA_SPEED * time.delta_secs();
    let forward = transform.forward();
    let right = transform.right();
    let up = transform.up();

    if keyboard_input.pressed(KeyCode::KeyW){
        transform.translation += forward * move_direction; 
    }

    if keyboard_input.pressed(KeyCode::KeyS){
        transform.translation -= forward * move_direction; 
    }

    if keyboard_input.pressed(KeyCode::KeyA){
        transform.translation -= right * move_direction; 
    }

    if keyboard_input.pressed(KeyCode::KeyD){
        transform.translation += right * move_direction; 
    }

    if keyboard_input.pressed(KeyCode::Space){
        transform.translation += up * move_direction; 
    }
    
    if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ShiftLeft){
        transform.translation -= up * move_direction; 
    }

    // Lock cursor
    if keyboard_input.just_pressed(KeyCode::Escape){
        cursor_options.visible = !cursor_options.visible;
        cursor_options.grab_mode = match cursor_options.grab_mode {
            CursorGrabMode::None=>CursorGrabMode::Locked,
            _=>CursorGrabMode::None
        }
    }

}


fn get_correct_pos() ->(f32,f32){
    let mut rng = rand::rng();
    let x = rng.random_range(-HALF_SIZE..=HALF_SIZE);
    let z = rng.random_range(-HALF_SIZE..=HALF_SIZE);
    if x == 0.0 && z == 0.0 {
        return get_correct_pos();
    }
    else {
        return (x,z);
    }

}
