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

    let step_size = 0.5;
    let map_limit = 20.0;

    //draw lines along the z axis
    let mut x = -map_limit;
    while x <= map_limit {
        let mut z = -map_limit;
        while z < map_limit {
            let next_z = z + step_size;
            let start =  Vec3::new(x,get_terrain_height(x,z),z);
            let end  = Vec3::new(x,get_terrain_height(x,next_z),next_z);
            gizmos.line(start,end,color);
            z = next_z;
        }
        x += 1.0;
    }

    // draw lines along the x axis 
    let mut z_line = -map_limit;
    while z_line <= map_limit {
        let mut x_line = -map_limit;
        while x_line < map_limit {
            let next_x = x_line+step_size;
            let start = Vec3::new(x_line,get_terrain_height(x_line,z_line),z_line);
            let end = Vec3::new(next_x,get_terrain_height(next_x,z_line),z_line);
            gizmos.line(start,end,color);
            x_line = next_x;
        }
        z_line += 1.0;
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
    for mut transform in query.iter_mut() {
        let current_x = transform.translation.x;
        let current_z = transform.translation.z;
        let ground_y =  get_terrain_height(current_x,current_z);
        let height_offset = 0.5;
        transform.translation.y = ground_y+height_offset;
    }
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
