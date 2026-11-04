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
    mut mutations:ResMut<Mutations>,
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

        if food_distance > EAT_DISTANCE + skeleton.skull_bone_scale {

            if final_displacement != Vec3::ZERO {
                let final_dir = final_displacement.normalize();

                let target_rotation = 
                    Transform::IDENTITY 
                    .looking_to(-final_dir,Vec3::Y)
                    .rotation;
                transform.rotation = transform.rotation.slerp(
                    target_rotation,
                    5.0*time.delta_secs()
                    );

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

                    let probability_leg =  rng.random_range(-0.5..=0.5);
                    if probability_leg != 0.0 {
                        println!("A mutation happened in the leg of juliana's baby");
                        mutations.count += 1;
                    }
                    let probability_skull = rng.random_range(-0.5..=0.5);
                    if probability_leg != 0.0 {
                        println!("A mutation happened in the skull of juliana's baby");
                        mutations.count += 1;
                    }
                    let probability_scale = rng.random_range(-0.2..=0.4);
                    if probability_leg != 0.0 {
                        println!("A mutation has happened in the scale of juliana's baby");
                        mutations.count += 1;
                    }
                        
                    let mut baby_skeleton = SkeletalDNA{
                        leg_bone_length:skeleton.leg_bone_length + probability_leg,
                        skull_bone_scale:skeleton.skull_bone_scale + probability_skull,
                        scale:skeleton.scale + probability_scale,
                    };
                    //lscale = life scale
                    let lscale = 1.0 + probability_scale;
                    for i in 0..4 {
                        let probability_brain = rng.random_range(-0.2..=0.2);
                        if probability_brain != 0.0 {
                            println!("A mutation has happened in the neural weights/brain of juliana's baby");
                            mutations.count += 1;
                        }
                        baby_brain.weights[i] += probability_brain ;
                        baby_brain.weights[i] = baby_brain.weights[i].clamp(-2.0,2.0);
                    }
                    let probability_speed = rng.random_range(0.0..=0.5);
                    if probability_speed != 0.0 {
                        println!("A mutation has happened in the speed of juliana's baby");
                        mutations.count += 1;
                    }
                        commands.spawn((
                            Transform::from_xyz(pos.x+EAT_DISTANCE+1.0,0.5,pos.z-2.0).with_scale(Vec3::splat(10.0*lscale)),
                            Juliana{
                                speed:JULIANA_SPEED - probability_speed,
                                current_energy:JDE * lscale,
                                max_energy:JME * lscale,
                                eaten_food:0,
                            },
                            baby_brain,
                            baby_skeleton.clone(),
                            WorldAssetRoot(
                                asset_server.load(
                                    GltfAssetLabel::Scene(0)
                                    .from_asset("trex/scene.gltf")
                                    )
                                ),
                        ));
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
    mut best_sucess:ResMut<BestSucess>,
    mut mutations:ResMut<Mutations>,
    asset_server:Res<AssetServer>,
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

            let target_rotation = Transform::IDENTITY 
                .looking_to(-direction.normalize(),Vec3::Y)
                .rotation;
            
            transform.rotation = transform.rotation.slerp(
                target_rotation,anti_juliana.speed*time.delta_secs()
                );
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
                    let probability_speed = rng.random_range(0.0..=0.5);
                    if probability_speed != 0.0 {
                        println!("A mutation has happened in the speed of Anti Juliana's baby");
                        mutations.count += 1;
                    }

                    commands.spawn((
                        WorldAssetRoot(
                            asset_server.load(
                                GltfAssetLabel::Scene(0)
                                .from_asset("cat/scene.gltf")
                                )
                            ),
                            Transform::from_xyz(pos.x+EAT_DISTANCE+1.0,1.0,pos.z-2.0).with_scale(Vec3::splat(7.0)),
                            AntiJuliana{
                                current_energy:AJDE,
                                max_energy:AJME,
                                speed:AJS-probability_speed,
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
                        let probability_speed = rng.random_range(0.0..=0.5);
                        if probability_speed != 0.0 {
                            println!("A mutation has happened in the speed of Anti Juliana's baby");
                            mutations.count += 1;
                        }
                        commands.spawn((
                            WorldAssetRoot(
                                asset_server.load(
                                    GltfAssetLabel::Scene(0)
                                    .from_asset("cat/scene.gltf")
                                    )
                                ),
                                Transform::from_xyz(pos.x+EAT_DISTANCE+1.0,1.0,pos.z-2.0).with_scale(Vec3::splat(7.0)),
                                AntiJuliana{
                                    current_energy:AJDE,
                                    max_energy:AJME,
                                    speed:AJS-probability_speed,
                                    attack_power:AJDR,
                                }

                        ));


                    }

                }
            }
            
        }
    }
}


