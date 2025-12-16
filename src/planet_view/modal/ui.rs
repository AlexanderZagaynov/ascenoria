use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::planet_view::types::colors;
use super::components::PlanetInfoModalButton;
use super::state::PlanetInfoModalState;

/// Spawn the modal panel content.
pub(crate) fn spawn_modal_panel(parent: &mut ChildSpawnerCommands, modal_state: &PlanetInfoModalState) {
    // Modal panel with starfield-like dark background
    parent
        .spawn((
            Node {
                width: Val::Px(380.0),
                min_height: Val::Px(220.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(0.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.02, 0.02, 0.06)),
            BorderColor::all(colors::BORDER),
            BorderRadius::all(Val::Px(4.0)),
        ))
        .with_child((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(3.0),
                ..default()
            },
            BackgroundColor(colors::BORDER),
            BorderRadius::top(Val::Px(4.0)),
        ))
        .with_children(|panel| {
            // Main content area with starfield background effect
            panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::new(
                            Val::Px(30.0),
                            Val::Px(30.0),
                            Val::Px(35.0),
                            Val::Px(25.0),
                        ),
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ))
                .with_children(|content| {
                    // Title line: "Icarus I Prosperity: 1 per day"
                    content.spawn((
                        Text::new(format!(
                            "{} Prosperity: {} per day",
                            modal_state.planet_name, modal_state.prosperity_per_day
                        )),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(colors::HEADER_TEXT),
                    ));

                    // Population growth info
                    let growth_text = if modal_state.days_to_growth > 0 {
                        format!(
                            "Population will grow in {} days.",
                            modal_state.days_to_growth
                        )
                    } else {
                        "Population at maximum capacity.".to_string()
                    };

                    content.spawn((
                        Text::new(growth_text),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));

                    // Current population status
                    content.spawn((
                        Text::new(format!(
                            "Population: {} / {}",
                            modal_state.population, modal_state.max_population
                        )),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.6, 0.65)),
                    ));
                });

            // Bottom border line
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(2.0),
                    ..default()
                },
                BackgroundColor(colors::BORDER),
            ));

            // OK button row
            panel
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|button_row| {
                    button_row
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(36.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.2, 0.25)),
                            BorderColor::all(colors::BORDER),
                            BorderRadius::all(Val::Px(3.0)),
                            PlanetInfoModalButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("OK"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(colors::HEADER_TEXT),
                            ));
                        });
                });
        });
}
