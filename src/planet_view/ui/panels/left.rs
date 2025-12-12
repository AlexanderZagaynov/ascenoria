use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::planet_view::types::colors;

/// Spawn the left information panel.
pub fn spawn_left_panel(
    main: &mut ChildSpawnerCommands,
    _planet_name: &str,
    _surface_type: &str,
    _planet_size: &str,
    surface_slots: usize,
    orbital_slots: usize,
) {
    main.spawn((
        Node {
            width: Val::Px(220.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            row_gap: Val::Px(10.0),
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(colors::PANEL_BG.with_alpha(0.85)),
        BorderColor::all(colors::BORDER),
    ))
    .with_children(|panel| {
        // Surface info header
        panel.spawn((
            Text::new("Surface"),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(colors::HEADER_TEXT),
        ));

        // Stats
        for (label, value) in [
            ("Slots", format!("{}", surface_slots)),
            ("Orbitals", format!("{}", orbital_slots)),
        ] {
            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));
                    row.spawn((
                        Text::new(value),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(colors::HEADER_TEXT),
                    ));
                });
        }

        // Divider
        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(colors::BORDER),
        ));

        // Population section
        panel.spawn((
            Text::new("Population"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::HEADER_TEXT),
        ));

        panel
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|pop_row| {
                for _ in 0..3 {
                    pop_row.spawn((
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.4, 0.6, 0.9)),
                    ));
                }
            });

        // Divider
        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(colors::BORDER),
        ));

        // Project section
        panel.spawn((
            Text::new("Project"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::HEADER_TEXT),
        ));
        panel.spawn((
            Text::new("None"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT.with_alpha(0.6)),
        ));

        // Controls at bottom
        panel
            .spawn((Node {
                margin: UiRect::top(Val::Auto),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },))
            .with_children(|controls| {
                controls.spawn((
                    Text::new("ESC - Return"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(colors::TEXT.with_alpha(0.5)),
                ));
            });
    });
}
