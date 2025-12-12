use bevy::prelude::*;
use crate::galaxy_view::colors;

/// Spawn the turn indicator rings at the top of the panel.
pub fn spawn_turn_indicators(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        })
        .with_children(|row| {
            // 5 ring indicators (like in Ascendancy)
            for _ in 0..5 {
                row.spawn((
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderColor::all(colors::RING_GREEN),
                    BorderRadius::all(Val::Percent(50.0)),
                ));
            }
        });
}
