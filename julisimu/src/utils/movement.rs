use bevy::prelude::*;
use super::data::*;
use rand::prelude::*;
use bevy::gltf::GltfAssetLabel;

pub fn move_juliana(
    mut commands:Commands,
    time:Res<Time>,
    food:Query<(&Transform,Entity),(With<Food>,Without<Juliana>)>,
    mut julianas:Query<(&mut Transform,&mut Juliana)>
    ) {
    for (mut transform ,mut juliana) in julianas.iter_mut() {
        let juliana_transform = transform.translation;
        let mut julia = juliana;
        let nearest = food.iter().map(|(f,entity)| (f.translation,entity)).min_by(|a,b|{
            a.0.distance_squared(juliana_transform)
                .total_cmp(&b.0.distance_squared(juliana_transform))
        });
        let Some(target) = nearest else {continue};
        let (target_pos,entity_id) = target;
        let mut direction = target_pos - juliana_transform;
        direction.y = 0.0;
        let distance = direction.length();
        if distance > 0.1 {
            let speed = julia.speed * direction.normalize();
            transform.translation += speed * time.delta_secs();
        }
        else {
            // eat 
            if julia.current_energy < julia.max_energy {
                julia.current_energy += 40.0;
            }
            else {
                // spawn another juliana
                println!("Spawning juliana's child");
                commands.spawn((
                    WorldAssetRoot(asset_server.load("bacteria.glb#Scene0")),
                    // TODO:alter the position in which juliana 2 will be spawned,but makes ure it
                    // cant spawn out of bounds,make juliana 2 spawn behind mother juliana
                    Transform::from_xyz(juliana_transform.x,0.5,juliana_transform.z).with_scale(Vec3::splat(0.01)),
                    Juliana{
                        speed:JULIANA_SPEED,
                        current_energy:JDE,
                        max_energy:JME
                }
                ));
            }
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

pub fn starve_juliana(
    time:Res<Time>,
    mut commands:Commands,
    mut julianas:Query<(Entity,&mut Juliana)>
    ) {
    for (entity,mut juliana) in julianas.iter_mut() {
        juliana.current_energy -= 10.0 * time.delta_secs();

        if juliana.current_energy <= 0.0 {
            println!("JULIANA DESPAWNED");
            commands.entity(entity).despawn();
        }
    } 

}


pub fn change_food_color (
    time:Res<Time>,
    mut food:Query<&MeshMaterial3d<StandardMaterial>,(With<Food>,Without<Juliana>)>,
    mut materials:ResMut<Assets<StandardMaterial>>
    ) {
    let hue = (time.elapsed_secs() * 60.0) % 360.0;
    let color = LinearRgba::from(Hsla::new(hue,1.0,0.5,1.0));
    for material_handle in food.iter() {
        if let Some(mut material) = materials.get_mut(material_handle) {
            material.base_color = Color::from(color);
            material.emissive = color * 10.0;
        }
    }
}
