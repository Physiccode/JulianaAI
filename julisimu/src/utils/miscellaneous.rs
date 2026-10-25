use bevy::prelude::*;
use rand::prelude::*;
use super::data::*;

pub fn evolution_manager(
    mut commands:Commands,
    asset_server:Res<AssetServer>,
    julianas:Query<(),With<Juliana>>,
    best_brain:Res<BestBrain>
    ) {
    if julianas.is_empty() {
        println!("Extinsion occured.Spawning new generation");
        for _ in 0..5 {
            let mut rng = rand::rng();
            let x = rng.random_range(-HALF_SIZE..=HALF_SIZE);
            let z = rng.random_range(-HALF_SIZE..=HALF_SIZE);
            let mut brain = best_brain.brain;
            for weight in brain.weights.iter_mut() {
                let mutation = rng.random_range(-2.0..=2.0);
                *weight = (*weight+mutation).clamp(-1.0,1.0);
            }
            commands.spawn((
                    WorldAssetRoot(asset_server.load("bacteria.glb#Scene0")),
                    Transform::from_xyz(x, 0.5, z).with_scale(Vec3::splat(0.01)),
                    Juliana {
                        speed: JULIANA_SPEED,
                        current_energy: JDE,
                        max_energy: JME,
                        eaten_food:0
                    },
                    // Gives each new starter a unique set of randomized preference weights
                    brain,
                ));
        }

    }
    
}
