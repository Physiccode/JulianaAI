use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use crate::utils::data::*;

pub fn create_ground(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>
    ) {
    let terrain_mesh = Plane3d::default()
        .mesh()
        .size(100.0,100.0)
        .subdivisions(50);
    let mut mesh_handle = meshes.add(terrain_mesh);
    if let Some(mut mesh) = meshes.get_mut(&mesh_handle){
        if let Some(VertexAttributeValues::Float32x3( positions)) = 
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION){
                for pos in positions.iter_mut() {
                            pos[1] = get_terrain_height(pos[0],pos[2]);
                        }
        }
        
    }
    commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(materials.add(StandardMaterial{
                base_color:Color::from(bevy::color::palettes::tailwind::STONE_800),
                perceptual_roughness:0.85,
                metallic:0.05,
                reflectance:0.2,
                ..default()
            })),
            Transform::from_xyz(0.0,0.0,0.0)
            ));
}


pub fn get_terrain_height(x:f32,z:f32)->f32{
    let wave1 = (x*0.1).sin() * (z*0.1).cos()*1.5;
    let wave2 = (x*0.05).sin() * (z*0.05).sin()*2.0;
    wave1+wave2
}

pub fn ground_entities(
    mut query:Query<&mut Transform,Or<(With<Juliana>,With<AntiJuliana>,With<Food>)>>
    ){
    for mut transform in query.iter_mut() {
        let current_x = transform.translation.x;
        let current_z = transform.translation.z;
        let ground_y =  get_terrain_height(current_x,current_z);
        let height_offset = 0.5;
        transform.translation.y = ground_y+height_offset;
    }
}
