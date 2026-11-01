use bevy::prelude::*;
use crate::utils::data::*;
use super::terrain::get_terrain_height;
use rand::prelude::*;
use rand::seq::IndexedRandom;
use bevy::gltf::GltfMesh;

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

//TODO:WAIT A RANDOM AMMOUNT OF TIME,AND THEN DESPAWN A RANDOM TREE,ONLY SPAWN TREES IF WE HAVEN'T
//REACHED THE LIMIT
// TODO:SPAWN TREES NEXT TO FOOD
pub fn spawn_food(
    food:Query<(),With<Food>>,
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    asset_server:Res<AssetServer>,
    forest_pack:Res<ForestPack>,
    gltf_assets:Res<Assets<Gltf>>,
    gltf_mesh_assets:Res<Assets<GltfMesh>>
    ) {


    let fallback_material = materials.add(StandardMaterial {
        base_color:Color::from(bevy::color::palettes::tailwind::GREEN_800),
        perceptual_roughness:0.85,
        ..default()
    });
    let Some(gltf) = gltf_assets.get(&forest_pack.container) else {return;};
    let mut rng = rand::rng();
    let mut count = food.iter().count();
    while count < MAX_FOOD {
        let x = rng.random_range(-HALF_SIZE..=HALF_SIZE);
        let z = rng.random_range(-HALF_SIZE..=HALF_SIZE);
        let random_index = rng.random_range(0..=5);
        let tree_path = format!("forest_pack.glb#Scene{}",random_index);
        let ground_y =  get_terrain_height(x,z);
        commands.spawn((
            Food,
            Mesh3d(meshes.add(Sphere::default().mesh().uv(32,18))),
            MeshMaterial3d(materials.add(StandardMaterial{
                base_color:Color::from(bevy::color::palettes::tailwind::BLUE_500),
                ..default()
            })),
            Transform::from_xyz(x,ground_y,z)   
            ));

        if let Some(mesh_handle) =  gltf.meshes.choose(&mut rng) {
            if let Some(gltf_mesh) = gltf_mesh_assets.get(mesh_handle) {
                if let Some(primitive) = gltf_mesh.primitives.first() {
                    let tree_material = if let Some(gltf_mat_handle) = &primitive.material {
                        if let Some(path) = gltf_mat_handle.path() {
                            let std_material_path = format!("{}/std",path.to_string());
                            asset_server.load(&std_material_path)
                        }
                        else {
                            fallback_material.clone()
                        }
                    }
                    else {
                        fallback_material.clone()
                    };
                    commands.spawn((
                        Mesh3d(primitive.mesh.clone()),
                        MeshMaterial3d(tree_material),
                        Transform::from_xyz(x+1.0,0.5,z+1.0)
                        .with_scale(Vec3::splat(0.5))
                        .with_rotation(Quat::from_rotation_x(-90.0_f32.to_radians())),
                    ));
                }
            }
    
        }
        
        count += 1;
    }
}
