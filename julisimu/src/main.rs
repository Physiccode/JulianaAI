use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin,DiagnosticsStore},
};
mod utils;
use utils::camera::{setup,move_camera};
use utils::movement::*;
use utils::terrain::*;
use utils::food::*;
use utils::miscellaneous::evolution_manager;
use crate::utils::data::BestSucess;
use crate::utils::data::BestBrain;

//TODO:ADD AN ENTITY COUNTER AND A JULIANA COUNTER AT THE RIGHT TOP PART OF THE SCREEN
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
        .add_systems(Update,(
                move_camera,
                spawn_food,
                move_juliana,
                move_predator,
                starve_juliana,
                change_food_color,
                evolution_manager,
                create_ground,
                ground_entities
                ))
        .insert_resource(BestBrain::default())
        .insert_resource(BestSucess::default())
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


