//! Galaxy panel UI for the game options screen.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::game_options::types::GalaxyInfoText;
use crate::game_options::ui::colors;

/// Spawns the galaxy preview panel on the left side.
pub fn spawn_galaxy_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(140.0), // Space for bottom bar
                },
                ..default()
            },
            BackgroundColor(colors::BACKGROUND),
        ))
        .with_children(|panel| {
            // Galaxy preview area - stars visualization
            panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(300.0),
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND),
                    BorderColor::all(colors::PANEL_BORDER),
                ))
                .with_children(|preview| {
                    // Placeholder stars
                    for _ in 0..30 {
                        let x = rand::random::<f32>() * 90.0 + 5.0;
                        let y = rand::random::<f32>() * 90.0 + 5.0;
                        let size = rand::random::<f32>() * 3.0 + 1.0;
                        let star_colors = [
                            Color::srgb(1.0, 0.3, 0.2),
                            Color::srgb(0.2, 0.5, 1.0),
                            Color::srgb(1.0, 0.9, 0.5),
                            Color::srgb(0.9, 0.9, 0.9),
                        ];
                        let color = star_colors[rand::random::<usize>() % star_colors.len()];

                        preview.spawn((
                            Node {
                                width: Val::Px(size),
                                height: Val::Px(size),
                                position_type: PositionType::Absolute,
                                left: Val::Percent(x),
                                top: Val::Percent(y),
                                ..default()
                            },
                            BackgroundColor(color),
                            BorderRadius::all(Val::Px(size / 2.0)),
                        ));
                    }
                });

            // Galaxy info text
            panel.spawn((
                Text::new("Average Star Cluster\nFive Species\nNeutral Atmosphere"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::INFO),
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                GalaxyInfoText,
            ));
        });
}
