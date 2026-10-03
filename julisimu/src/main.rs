use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin,DiagnosticsStore},
    input::mouse::AccumulatedMouseMotion,
    gltf::GltfAssetLabel,
    light::PointLight
};

#[derive(Component)]
struct MovableCamera;

const CAMERA_SENSIVITY:f32 = 0.003;
const CAMERA_SPEED:f32 = 7.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window:Some(Window{
                title:"JulianaAI-Evolutionism simulation".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup,setup)
        .add_systems(Update,(move_camera,draw_grid))
        .run();
}

// TODO:turn this into a function that sends the diagnostic to my esp32
fn print_diagnostic(diagnostics:Res<DiagnosticsStore>) {
   if let Some (fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
       if let Some(fps) = fps_diagnostic.value(){
           let current_fps:f64 = fps ;
           println!("fps:{current_fps}");
       }
   } 
}

fn setup(
    mut commands:Commands,
    asset_server:Res<AssetServer>,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>
    ) {
   //Spawn camera
   commands.spawn((
           Camera3d::default(),
           Transform::from_xyz(0.0,2.0,4.0)
            .looking_at(Vec3::ZERO,Vec3::Y)
           ,
           MovableCamera
           ));

   // Spawn light 
   commands.spawn((PointLight{
       intensity:500_000.0,
       shadow_maps_enabled:true,
       ..default()},
       Transform::from_xyz(0.0,0.0,0.0),
   ));
   
   // spawn sphere 
   commands.spawn((
           Mesh3d(meshes.add(Sphere::default().mesh().uv(32,18))),
           MeshMaterial3d(materials.add(StandardMaterial{
               base_color:Color::from(bevy::color::palettes::tailwind::BLUE_500),
               ..default()
           })),
          Transform::from_xyz(0.0,0.5,0.0)   
           ));
//   let cell_handle:Handle<WorldAsset> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cell.glb"));
}

fn move_camera(
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

}

fn draw_grid(mut gizmos:Gizmos) {
   let color = Color::srgb(0.35,0.35,0.35); 
   for i in -10..=10 {
       let p = i as f32;
       gizmos.line(Vec3::new(p,0.0,-10.0),Vec3::new(p,0.0,10.0),color);
       gizmos.line(Vec3::new(-10.0,0.0,p),Vec3::new(10.0,0.0,p),color);
   }
}
