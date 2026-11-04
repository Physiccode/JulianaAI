use bevy::prelude::*;
use super::data::*;

pub fn setup_text(mut commands:Commands) {
    commands.spawn(Node{
        position_type:PositionType::Absolute,
        top:Val::Px(0.0),
        left:Val::Px(0.0),
        margin:UiRect::all(Val::Px(0.0)),
        padding:UiRect::all(Val::Px(0.0)),
        flex_direction:FlexDirection::Column,
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            SimulationText,
            Text::new(""),
            TextColor(Color::from(bevy::color::palettes::tailwind::SLATE_500)),
            TextFont{
                font_size:FontSize::Px(16.0),
                ..default()
            }
            ));
    });
}

pub fn update_text(
    juliana_queries:Query<Entity,With<Juliana>>,
    anti_julianas:Query<Entity,With<AntiJuliana>>,
    food:Query<Entity,With<Food>>,
    mutations:Res<Mutations>,
    dead_julianas:Res<DeadJulianas>,
    julianas:Query<(&Juliana,&Brain)>,
    mut text_query:Query<&mut Text,With<SimulationText>>,
    ) {

    let count = juliana_queries.iter().count();
    let ajcount = anti_julianas.iter().count();
    let food = food.iter().count();
    let mutations_count = mutations.count;
    let dead_julianas = dead_julianas.count;
    let apex_survivor = julianas.iter().max_by_key(|(juliana,_)| juliana.eaten_food);
    
    if let Ok(mut text) = text_query.single_mut() {
        if let Some((juliana,brain)) = apex_survivor {
            **text = format!("
                    Julianas:{count}
                    Anti-julianas:{ajcount}
                    Food:{food}
                    Mutations:{mutations_count}
                    Dead julianas:{dead_julianas}
                    
                    Best neural network:has eaten {} spheres
                    Weights:
                    [ {:.4} , {:.4} ]
                    [ {:.4} , {:.4} ]
                        ",juliana.eaten_food,
                        brain.weights[0],brain.weights[1],brain.weights[2],brain.weights[3]
                        );
        }
        else {
            **text = format!("
                    Julianas:{count}
                    Anti-julianas:{ajcount}
                    Food:{food}
                    Mutations:{mutations_count}
                    Dead julianas:{dead_julianas}
                        ");

        }
    }
}

//Add a button that fixex the camera position
//Show longest leg length
//Show the best neural network to screen
//Make it so i can toggle when the text is off/on
