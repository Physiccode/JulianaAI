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
                Transform::from_xyz(x,0.5,z).with_scale(Vec3::splat(1.0)),
                Juliana {
                        speed: JULIANA_SPEED,
                        current_energy: JDE,
                        max_energy: JME,
                        eaten_food:0
                    },
                brain,
                SkeletalDNA::default(),
            )).with_children(|parent|{
                parent.spawn((
                        Mesh3d(meshes.add(Cylinder::new(0.1,1.0))),
                        MeshMaterial3d(materials.add(StandardMaterial{
                            base_color:Color::from(bevy::color::palettes::tailwind::GRAY_400),
                            ..default()
                        })),
                        Transform {
                            translation:Vec3::new(-0.4,-0.2,0.0),
                            scale:Vec3::new(1.0,1.0,1.0),
                            ..default()
                        }
                        ));
                // right leg
                parent.spawn((
                        Mesh3d(meshes.add(Cylinder::new(0.1,1.0))),
                        MeshMaterial3d(materials.add(StandardMaterial{
                            base_color:Color::from(bevy::color::palettes::tailwind::GRAY_400),
                            ..default()
                        })),
                        Transform {
                            translation:Vec3::new(0.4,-0.2,0.0),
                            scale:Vec3::new(1.0,1.0,1.0),
                            ..default()
                        }
                        ));

            });
        }

    }
    
}
