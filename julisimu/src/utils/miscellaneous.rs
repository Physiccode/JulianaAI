use bevy::prelude::*;
use rand::prelude::*;
use super::data::*;

pub fn evolution_manager(
    mut commands:Commands,
    asset_server:Res<AssetServer>,
    julianas:Query<(),With<Juliana>>,
    best_brain:Res<BestBrain>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    mut meshes:ResMut<Assets<Mesh>>

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
                WorldAssetRoot(
                    asset_server.load(
                        GltfAssetLabel::Scene(0)
                        .from_asset("trex/scene.gltf")
                        )
                    ),
                    Transform::from_xyz(x,0.0,z)
                    .with_scale(Vec3::splat(10.0)),
                Juliana {
                        speed: JULIANA_SPEED,
                        current_energy: JDE,
                        max_energy: JME,
                        eaten_food:0
                    },
                brain,
                SkeletalDNA::default(),
            ));
        }

    }
    
}
