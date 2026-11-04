use bevy::prelude::*;
use super::data::*;

pub fn setup_text(mut commands:Commands) {
    commands.spawn((
            SimulationText,
            Text::new(""),
            TextColor(Color::from(bevy::color::palettes::tailwind::SLATE_100)),
            Node {
                position_type:PositionType::Absolute,
                top:px(6),
                left:px(5),
                ..default()
            },
            ));
}

pub fn update_text(
    juliana_queries:Query<Entity,With<Juliana>>,
    anti_julianas:Query<Entity,With<AntiJuliana>>,
    food:Query<Entity,With<Food>>,
    mut text_query:Query<&mut Text,With<SimulationText>>,
    ) {

    let count = juliana_queries.iter().count();
    let ajcount = anti_julianas.iter().count();
    let food = food.iter().count();

    if let Ok(mut text) = text_query.single_mut() {
        **text = format!("
        Julianas:{count}
        Anti-julianas:{ajcount}
        Food:{food}
            ");
    }
}

//Show the best neural network to screen
//Make it so i can toggle when the text is off/on
