use bevy::prelude::*;
use super::data::*;
use rand::prelude::*;
use bevy::gltf::GltfAssetLabel;

pub fn move_juliana(
    mut commands:Commands,
    time:Res<Time>,
    food:Query<(&Transform,Entity),(With<Food>,Without<Juliana>)>,
    mut julianas:Query<&mut Transform,With<Juliana>>
    ) {
    for mut juliana in julianas {
        let nearest = food.iter().map(|(f,entity)| (f.translation,entity)).min_by(|a,b|{
            a.0.distance_squared(juliana.translation)
                .total_cmp(&b.0.distance_squared(juliana.translation))
        });
        let Some(target) = nearest else {continue};
        let (target_pos,entity_id) = target;
        let mut direction = target_pos - juliana.translation;
        direction.y = 0.0;
        let distance = direction.length();
        if distance > 0.1 {
            let speed = JULIANA_SPEED * direction.normalize();
            juliana.translation += speed * time.delta_secs();
        }
        else {
            // eat 
            commands.entity(entity_id).despawn();
        }
    }
}

pub fn spawn_food(
    food:Query<(),With<Food>>,
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>
    ) {
    let mut rng = rand::rng();
    let mut count = food.iter().count();
    while count < MAX_FOOD {
        let x = rng.random_range(-HALF_SIZE..=HALF_SIZE);
        let z = rng.random_range(-HALF_SIZE..=HALF_SIZE);

       //TODO:make the food change color smoothly and glow
        commands.spawn((
                Food,
                Mesh3d(meshes.add(Sphere::default().mesh().uv(32,18))),
                MeshMaterial3d(materials.add(StandardMaterial{
                    base_color:Color::from(bevy::color::palettes::tailwind::BLUE_500),
                    ..default()
                })),
                Transform::from_xyz(x,0.5,z)   
                ));

        count += 1;
    }
}


