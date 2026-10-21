use bevy::prelude::*;
use super::data::*;
use rand::prelude::*;
use bevy::gltf::GltfAssetLabel;
use std::collections::HashSet;

pub fn move_juliana(
    mut commands:Commands,
    time:Res<Time>,
    food:Query<(&Transform,Entity),(With<Food>,Without<Juliana>)>,
    asset_server:Res<AssetServer>,
    mut julianas:Query<(&mut Transform,&mut Juliana,&Brain)>
    predators:Query<&Transform,With<AntiJuliana>>
    ) {
    let mut eaten = HashSet::new();
    
    for (mut transform ,mut juliana,brain) in julianas.iter_mut() {

        let juliana_transform = transform.translation;
        let mut julia = juliana;

        let nearest_food = food.iter()
            .filter(|(_,entity_id)| !eaten.contains(entity_id))
            .map(|(f,entity)| (f.translation,entity)).min_by(|a,b|{
            a.0.distance_squared(juliana_transform)
                .total_cmp(&b.0.distance_squared(juliana_transform))
        });
        let Some(food_target) = nearest_food else {continue};
        let (food_target_pos,entity_id) = food_target;

        let food_direction = (food_target_pos - juliana_transform).normalize_or_zero().unwrap_or(Vec3::ZERO);
        let food_distance = food_direction.length();

        let nearest_aj = predators.iter().min_by(|a,b|{
            a.translation.distance_squared(juliana_transform)
                .total_cmp(&b.translation.distance_sqiared(juliana_transform))
        });

        let Some(aj_pos) = nearest_aj else {continue};
        let aj_dir = (aj_pos - juliana_transform).normalize_or_zero().unwrap_or(Vec3::ZERO);
        // multiply directions by genetic weight
        let final_x = (food_direction.x * brain.weights[0] + aj_dir.x * brain.weights[2]);
        let final_z = (food_direction.z * brain.weights[1] + aj_dir.x * brain.weights[3]);
        let final_displacement = Vec3::new(final_x,0.0,final_z);

        if food_distance > EAT_DISTANCE {
            if final_displacemenet != Vec3::ZERO {
                let final_dir = final_dir.normalize();
                transform.translation += final_dir * julia.speed * time.delta_secs(); 
            }

        }
        else {
            // eat 
            if eaten.insert(entity_id) {

                commands.entity(entity_id).despawn();

                if julia.current_energy < julia.max_energy {
                    julia.current_energy += 40.0;
                }
                else  { // spawn another juliana 
                    // Lose energy during birth:
                    julia.current_energy -= 80.0;
                    println!("Spawning juliana's child");
                    let pos = transform.translation;
                    // potentially reduce juliana's children's speed
                    let mut baby_brain = brain.clone();
                    for i in 0..4 {
                        baby_brain.weights[i] += rng.random_range(-0.2..=0.2);
                        baby_brain.weights = baby_brain.weights[i].clamp(-2.0,2.0);
                    }
                    let mut rng = rand::rng();
                    let speed_reducer = rng.random_range(0.0..=0.5);
                    commands.spawn((
                        WorldAssetRoot(asset_server.load("bacteria.glb#Scene0")),
                        Transform::from_xyz(pos.x+EAT_DISTANCE+1.0,0.5,pos.z -2.0).with_scale(Vec3::splat(0.01)),
                        Juliana{
                            speed:JULIANA_SPEED - speed_reducer,
                            current_energy:JDE,
                            max_energy:JME
                        },
                    ));
                }
            }
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
        juliana.current_energy -= STARVATION_RATE * time.delta_secs();

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


pub fn move_predator(
    mut commands:Commands,
    time:Res<Time>,
    mut julianas:Query<(&Transform,Entity,&mut Juliana),(With<Juliana>,Without<AntiJuliana>)>,
    mut predators:Query<(&mut Transform,&mut AntiJuliana),(With<AntiJuliana>,Without<Juliana>)>,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>
    ){
    let mut eaten = HashSet::new();
    
    for (mut transform,mut anti_juliana) in predators.iter_mut() {
        let predator_pos = transform.translation;

        let nearest = julianas.iter_mut()
            .filter(|(_,entity_id,_)| !eaten.contains(entity_id))
            .map(|(aj_pos,entity,j)|(aj_pos.translation,entity,j)).min_by(|a,b|{
            a.0.distance_squared(predator_pos)
                .total_cmp(&b.0.distance_squared(predator_pos))
            });
        let Some(target) = nearest else {continue};
        let (target_pos,entity_id,mut target_juliana) = target;
        let mut direction = target_pos - predator_pos;
        let distance = direction.length();
        if distance > EAT_DISTANCE {
            let speed = anti_juliana.speed * direction.normalize();
            transform.translation += speed * time.delta_secs();
        }
        else {
            if target_juliana.current_energy > AJDR {
                target_juliana.current_energy -= AJDR;
                if anti_juliana.current_energy < AJME {
                    anti_juliana.current_energy += 10.0;
                }
                else {
                    // Spawn baby anti-juliana
                    anti_juliana.current_energy -= 100.0;
                    println!("Spawning anti-juliana's child");
                    let pos = transform.translation;
                    let mut rng = rand::rng();
                    let speed_reducer = rng.random_range(0.0..=0.5);
                    commands.spawn((
                            Mesh3d(meshes.add(Capsule3d::default())),
                            MeshMaterial3d(materials.add(StandardMaterial{
                                base_color:Color::from(bevy::color::palettes::tailwind::FUCHSIA_950),
                                unlit:false,
                                ..default()
                            })),
                            Transform::from_xyz(pos.x+EAT_DISTANCE+1.0,1.0,pos.z-2.0),
                            AntiJuliana{
                                current_energy:AJDE,
                                max_energy:AJME,
                                speed:AJS-speed_reducer,
                                attack_power:AJDR,
                            }

                        ));

                }
            }
            else {
                // eat and kill juliana
                if eaten.insert(entity_id) {

                    commands.entity(entity_id).despawn();

                    if anti_juliana.current_energy < anti_juliana.max_energy {
                        anti_juliana.current_energy += 10.0;     
                    }
                    else {
                        // Spawn julia baby
                        anti_juliana.current_energy -= 100.0;
                        println!("Spawning anti-juliana's child");
                        let pos = transform.translation;
                        let mut rng = rand::rng();
                        let speed_reducer = rng.random_range(0.0..=0.5);
                        commands.spawn((
                                Mesh3d(meshes.add(Capsule3d::default())),
                                MeshMaterial3d(materials.add(StandardMaterial{
                                    base_color:Color::from(bevy::color::palettes::tailwind::FUCHSIA_950),
                                    unlit:false,
                                    ..default()
                                })),
                                Transform::from_xyz(pos.x+EAT_DISTANCE+1.0,1.0,pos.z-2.0),
                                AntiJuliana{
                                    current_energy:AJDE,
                                    max_energy:AJME,
                                    speed:AJS-speed_reducer,
                                    attack_power:AJDR,
                                }

                        ));


                    }

                }
            }
            
        }
    }
}
