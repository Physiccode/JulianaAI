use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin,DiagnosticsStore}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window:Some(Window{
                title:"JulianaAI-Evolutionism simulation".to_string(),
                ..default()
            }),
            ..default()
        }))
//        .add_systems(Startup,())
        .add_systems(Update,print_diagnostic)
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
