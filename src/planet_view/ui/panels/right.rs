//! Right panel for orbital structure management.
//!
//! Displays available orbital slots where the player can construct
//! space stations, defense platforms, and other orbital structures.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::planet_view::types::colors;

/// Spawn the right orbital structures panel.
///
/// # Layout
/// ```text
/// ┌──────────────────┐
/// │ Orbitals         │
/// │ [Slot 1 - empty] │
/// │ [Slot 2 - empty] │
/// │ [Slot 3 - empty] │
/// │ +N more          │
/// └──────────────────┘
/// ```
///
/// Shows up to 8 slots visually, with a "+N more" indicator
/// if there are additional slots beyond that.
pub fn spawn_right_panel(main: &mut ChildSpawnerCommands, orbital_slots: usize) {
    main.spawn((
        Node {
            width: Val::Px(180.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            row_gap: Val::Px(8.0),
            border: UiRect::left(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(colors::PANEL_BG.with_alpha(0.85)),
        BorderColor::all(colors::BORDER),
    ))
    .with_children(|panel| {
        panel.spawn((
            Text::new("Orbitals"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(colors::HEADER_TEXT),
        ));

        // Orbital slots display
        for i in 0..orbital_slots.min(8) {
            panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(28.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_NORMAL),
                    BorderColor::all(colors::BORDER),
                ))
                .with_children(|slot| {
                    slot.spawn((
                        Text::new(format!("Slot {}", i + 1)),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(colors::TEXT.with_alpha(0.5)),
                    ));
                });
        }

        if orbital_slots > 8 {
            panel.spawn((
                Text::new(format!("+{} more", orbital_slots - 8)),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(colors::TEXT.with_alpha(0.4)),
            ));
        }
    });
}
