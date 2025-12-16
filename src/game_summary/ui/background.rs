use bevy::prelude::*;
use super::super::types::{GameSummaryRoot, StarBackground};

/// Spawns decorative star background.
pub fn spawn_star_background(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            StarBackground,
            GameSummaryRoot,
            ZIndex(-1),
        ))
        .with_children(|parent| {
            // Spawn some decorative "stars" as small nodes
            let star_positions = [
                (10.0, 15.0, 2.0),
                (25.0, 8.0, 3.0),
                (40.0, 22.0, 2.0),
                (55.0, 5.0, 2.5),
                (70.0, 18.0, 2.0),
                (85.0, 12.0, 3.0),
                (15.0, 75.0, 2.0),
                (30.0, 85.0, 2.5),
                (50.0, 78.0, 2.0),
                (65.0, 90.0, 3.0),
                (80.0, 82.0, 2.0),
                (92.0, 70.0, 2.5),
                (5.0, 45.0, 2.0),
                (95.0, 40.0, 2.0),
                (88.0, 55.0, 2.5),
                (8.0, 60.0, 2.0),
            ];

            for (x, y, size) in star_positions {
                parent.spawn((
                    Node {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Percent(y),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.8, 0.8, 0.9, 0.6)),
                    BorderRadius::all(Val::Px(size / 2.0)),
                ));
            }
        });
}
