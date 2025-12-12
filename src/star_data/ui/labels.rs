use bevy::prelude::*;
use super::super::types::{StarRoot, colors};

/// Spawn system labels (player icons in corners).
pub fn spawn_system_label(commands: &mut Commands, _system_name: &str) {
    // Player species icon in corner
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::SELECTION_GREEN),
            StarRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::SELECTION_GREEN),
            ));
        });

    // Same icon in top-right (mirrored like in Ascendancy)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(230.0), // Account for panel width
                top: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::SELECTION_GREEN),
            StarRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::SELECTION_GREEN),
            ));
        });
}
