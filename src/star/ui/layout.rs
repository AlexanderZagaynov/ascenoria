use bevy::prelude::*;
use crate::galaxy::Galaxy;
use super::super::types::{StarRoot, colors};

use super::header::spawn_system_header;
use super::navigation::spawn_navigation_buttons;
use super::info::spawn_planet_info_area;
use super::controls::spawn_bottom_controls;

/// Spawn the right-side UI panel.
pub fn spawn_ui_panel(commands: &mut Commands, galaxy: &Galaxy, star_index: usize) {
    let system = galaxy.systems.get(star_index);
    let system_name = system.map(|s| s.name.as_str()).unwrap_or("Unknown");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            StarRoot,
        ))
        .with_children(|parent| {
            // Right panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_BG),
                ))
                .with_children(|panel| {
                    // System name header
                    spawn_system_header(panel, system_name);

                    // Navigation button row (like in screenshot)
                    spawn_navigation_buttons(panel);

                    // Planet info area
                    spawn_planet_info_area(panel, system);

                    // Spacer
                    panel.spawn(Node {
                        flex_grow: 1.0,
                        ..default()
                    });

                    // Bottom control buttons
                    spawn_bottom_controls(panel);
                });
        });
}
