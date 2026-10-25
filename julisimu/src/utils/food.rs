use bevy::prelude::*;
use crate::utils::data::*;
use rand::prelude::*;

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
