use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin,DiagnosticsStore},
};
mod utils;
use utils::camera::{setup,move_camera};
use utils::movement::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window:Some(Window{
                title:"JulianaAI-Compared anatomy simulation".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup,setup)
        .add_systems(Update,(move_camera,spawn_food,move_juliana,draw_grid,starve_juliana))
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

fn draw_grid(mut gizmos:Gizmos) {
   let color = Color::srgb(0.35,0.35,0.35); 
   for i in -20..=20 {
       let p = i as f32;
       gizmos.line(Vec3::new(p,0.0,-20.0),Vec3::new(p,0.0,20.0),color);
       gizmos.line(Vec3::new(-20.0,0.0,p),Vec3::new(20.0,0.0,p),color);
   }
}
