use bevy::prelude::*;
use super::data::*;
use rand::prelude::*;
use std::collections::HashSet;

pub fn move_juliana(
    mut commands:Commands,
    time:Res<Time>,
    food:Query<(&Transform,Entity),(With<Food>,Without<Juliana>)>,
    asset_server:Res<AssetServer>,
    mut julianas:Query<(&mut Transform,&mut Juliana,&Brain,&SkeletalDNA),(With<Juliana>,Without<AntiJuliana>)>,
    predators:Query<&Transform,(With<AntiJuliana>,Without<Juliana>)>,

    mut materials:ResMut<Assets<StandardMaterial>>,
    mut meshes:ResMut<Assets<Mesh>>,

    ) {
    let mut eaten = HashSet::new();
    
    for (mut transform ,mut juliana,brain,skeleton) in julianas.iter_mut() {

        let juliana_transform = transform.translation;
        let mut julia = juliana;
        let speed = julia.speed * skeleton.leg_bone_length;
        let energy_drain_multiplier = 1.0+(skeleton.leg_bone_length*0.5);

        let nearest_food = food.iter()
            .filter(|(_,entity_id)| !eaten.contains(entity_id))
            .map(|(f,entity)| (f.translation,entity)).min_by(|a,b|{
            a.0.distance_squared(juliana_transform)
                .total_cmp(&b.0.distance_squared(juliana_transform))
        });
        let Some(food_target) = nearest_food else {continue};
        let (food_target_pos,entity_id) = food_target;

        let food_dir = food_target_pos - juliana_transform;
        let food_direction = food_dir.normalize_or_zero();
        let food_distance = food_dir.length();

        let nearest_aj = predators.iter().min_by(|a,b|{
            a.translation.distance_squared(juliana_transform)
                .total_cmp(&b.translation.distance_squared(juliana_transform))
        });

        let Some(aj_pos) = nearest_aj else {continue};
        let aj_dir = (aj_pos.translation - juliana_transform).normalize_or_zero();
        // multiply directions by genetic weight
        // Pass throught tanh to add non-linearity
        let final_x = (food_direction.x * brain.weights[0] + aj_dir.x * brain.weights[2]).tanh();
        let final_z = (food_direction.z * brain.weights[1] + aj_dir.z * brain.weights[3]).tanh();
        let final_displacement = Vec3::new(final_x,0.0,final_z);

        if food_distance > EAT_DISTANCE {
            if final_displacement != Vec3::ZERO {
                let final_dir = final_displacement.normalize();
                transform.translation += final_dir * speed * time.delta_secs(); 

                transform.translation.x = transform.translation.x.clamp(-HALF_SIZE,HALF_SIZE);
                transform.translation.z = transform.translation.z.clamp(-HALF_SIZE,HALF_SIZE);


            }

        }
        else {
            // eat 
            if eaten.insert(entity_id) {

                commands.entity(entity_id).despawn();
                julia.eaten_food +=1;

                if julia.current_energy < julia.max_energy {
                    julia.current_energy += 40.0;
                }
                else  { // spawn another juliana 
                    // Lose energy during birth:
                    julia.current_energy -= 80.0;
                    println!("Spawning juliana's child");
                    let pos = transform.translation;
                    // potentially reduce juliana's children's speed
                    let mut rng = rand::rng();
                    let mut baby_brain = brain.clone();
                    let mut baby_skeleton = SkeletalDNA{
                        leg_bone_length:skeleton.leg_bone_length+rng.random_range(-0.5..=0.5),
                        head_bone_length:skeleton.head_bone_length+rng.random_range(-0.5..=0.5),
                    };
                    for i in 0..4 {
                        baby_brain.weights[i] += rng.random_range(-0.2..=0.2);
                        baby_brain.weights[i] = baby_brain.weights[i].clamp(-2.0,2.0);
                    }
                    let speed_reducer = rng.random_range(0.0..=0.5);
                        commands.spawn((
                            Transform::from_xyz(pos.x+EAT_DISTANCE+1.0,0.5,pos.z-2.0).with_scale(Vec3::splat(0.01)),
                            Juliana{
                                speed:JULIANA_SPEED - speed_reducer,
                                current_energy:JDE,
                                max_energy:JME,
                                eaten_food:0,
                            },
                            baby_brain,
                            baby_skeleton.clone(),
                        )).with_children(|parent|{
                            parent.spawn((
                                    Mesh3d(meshes.add(Cylinder::new(0.1,1.0))),
                                    MeshMaterial3d(materials.add(StandardMaterial{
                                        base_color:Color::from(bevy::color::palettes::tailwind::GRAY_400),
                                        ..default()
                                    })),
                                    Transform {
                                        translation:Vec3::new(-0.4,-0.2,0.0),
                                        scale:Vec3::new(1.0,baby_skeleton.leg_bone_length,1.0),
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
                                        scale:Vec3::new(1.0,baby_skeleton.leg_bone_length,1.0),
                                        ..default()
                                    }
                                    ));

                        });
                }
            }
        }
    }
}

pub fn move_predator(
    mut commands:Commands,
    time:Res<Time>,
    mut julianas:Query<(&Transform,Entity,&mut Juliana,&Brain),(With<Juliana>,Without<AntiJuliana>)>,
    mut predators:Query<(&mut Transform,&mut AntiJuliana),(With<AntiJuliana>,Without<Juliana>)>,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    mut best_brain:ResMut<BestBrain>,
    mut best_sucess:ResMut<BestSucess>
    ){
    let mut eaten = HashSet::new();
    
    for (mut transform,mut anti_juliana) in predators.iter_mut() {
        let predator_pos = transform.translation;

        let nearest = julianas.iter_mut()
            .filter(|(_,entity_id,_,_)| !eaten.contains(entity_id))
            .map(|(aj_pos,entity,j,brain)|(aj_pos.translation,entity,j,brain)).min_by(|a,b|{
            a.0.distance_squared(predator_pos)
                .total_cmp(&b.0.distance_squared(predator_pos))
            });
        let Some(target) = nearest else {continue};
        let (target_pos,entity_id,mut target_juliana,target_brain) = target;
        let direction = target_pos - predator_pos;
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

                    // Before killing juliana,print her brain component for diagnostic
                    println!("Juliana's brain:{:?}",target_brain.weights);
                    if target_juliana.eaten_food > best_sucess.count {
                        best_sucess.count = target_juliana.eaten_food;
                        best_brain.brain = *target_brain;
                        println!("Best brain until now:{:?},sucess count:{}",best_brain.brain,best_sucess.count);
                    }
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


