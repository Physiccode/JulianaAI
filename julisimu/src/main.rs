use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin,DiagnosticsStore},
//    light::DirectionalLightShadowMap,
    
};
//TODO:ORGANIZE THIS USE PILE
mod utils;
use utils::camera::{setup,move_camera};
use utils::movement::*;
use utils::terrain::*;
use utils::food::*;
use utils::text::*;
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
//        .insert_resource(bevy::light::DirectionalLightShadowMap{size:512})
        .add_systems(Startup,(setup,setup_background,setup_text))
        .add_systems(Update,(
                create_ground,
                move_camera,
                spawn_food,
                move_juliana,
                starve_juliana,
                move_predator,
                update_text,
                change_food_color,
                evolution_manager,
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


