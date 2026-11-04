use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use super::data::*;

pub fn create_ground(
    /*
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>
    */
    mut gizmos:Gizmos
    ) {

    /*
    let terrain_mesh = Plane3d::default()
        .mesh()
        .size(50.0,50.0)
        .subdivisions(30);
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
                perceptual_roughness:1.0,
                metallic:0.0,
                reflectance:0.0,
                alpha_mode:AlphaMode::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0,0.0,0.0)
            ));

*/

    let color = Color::srgb(0.35,0.35,0.35); 
    for i in -20..=20 {
        let p = i as f32;
        gizmos.line(Vec3::new(p,0.0,-20.0),Vec3::new(p,0.0,20.0),color);
        gizmos.line(Vec3::new(-20.0,0.0,p),Vec3::new(20.0,0.0,p),color);
    }
}


pub fn get_terrain_height( x:f32,z:f32) ->f32 {
    let wave1 = (x*0.1).sin() * (z*0.1).cos()*1.5;
    let wave2 = (x*0.05).sin() * (z*0.05).sin()*2.0;
    wave1+wave2
}

pub fn ground_entities(
    mut query:Query<&mut Transform,Or<(With<Juliana>,With<AntiJuliana>)>>
    ){
    /*
    for mut transform in query.iter_mut() {
        let current_x = transform.translation.x;
        let current_z = transform.translation.z;
        let ground_y =  get_terrain_height(current_x,current_z);
        let height_offset = 0.5;
        transform.translation.y = ground_y+height_offset;
    }
    */
}

pub fn setup_background(
    mut commands:Commands,
    asset_server:Res<AssetServer>,
    ) {
 // Add sunlight:
   commands.spawn((
           DirectionalLight{
               illuminance:5000.0,
               shadow_maps_enabled:false,
               ..default()
           },
           Transform::from_xyz(15.0,30.0,15.0).looking_at(Vec3::ZERO,Vec3::Y),

           ));
   // Add sky ambient light
   commands.insert_resource(GlobalAmbientLight{
       color:Color::from(bevy::color::palettes::tailwind::AMBER_50),
       brightness:150.0,
       ..default()
   });

  //Load forest tree pack
    commands.insert_resource(ForestPack{
        container:asset_server.load("forest_pack.glb"),
    });

}
