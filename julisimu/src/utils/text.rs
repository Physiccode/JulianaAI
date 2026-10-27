use bevy::prelude::*;
use super::data::*;

pub fn setup_text(mut commands:Commands) {
    commands.spawn((
            SimulationText,
            Text::new("Generation 1"),
            TextColor(BLUE.into()),
            Node {
                position_type:PositionType::Absolute,
                top:px(12),
                left:px(10),
                ..default()
            },
            ));
}
