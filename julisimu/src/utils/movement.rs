use bevy::prelude::*;
use super::data::*;
use rand::Rng;
use bevy::gltf::GltfAssetLabel;

pub fn move_juliana(
    time:Res<Time>
    food:Query<&Transform,(With<Food>,Without<Juliana>)>
    mut julianas:Query<&mut Transform,With<Juliana>>
    ) {
    for mut juliana in julianas {
        let nearest = food.iter().map(|f| f.translation).min_by(|a,b|{
            a.distance_squared(juliana.translation)
                .total_cmp(&b.distance_squared(juliana.translation))
        });
        let Some(target) = nearest else {continue};
        let mut dir = target - juliana.translation;
        dir.y = 0.0;
        let dist = dir.length();
        if dist < 0.001 {
            continue
        }



    }
}
pub fn eat(){

}

pub fn spawn_food(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>
    food:Query<(),With<Food>
    ) {
    let mut rng = rand::thread_rng();
    let mut count = food.iter().count();
    while count < MAX_FOOD {
       let x = rng.gen_range(-HALF_SIZE..HALF_SIZE);
       let y = rng.gen_range(-HALF_SIZE..HALF_SIZE);

       //TODO:make the food change color smoothly and glow
        commands.spawn((
                Food,
                Mesh3d(meshes.add(Sphere::default().mesh().uv(32,18))),
                MeshMaterial3d(materials.add(StandardMaterial{
                    base_color:Color::from(bevy::color::palettes::tailwind::BLUE_500),
                    ..default()
                })),
                Transform::from_xyz(0.0,0.25,0.0)   
                ));

    }
    count += 1;
}


