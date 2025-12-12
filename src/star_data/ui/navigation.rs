use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use super::super::types::{SystemPanelButton, colors};

pub fn spawn_navigation_buttons(panel: &mut ChildSpawnerCommands) {
    // Row of icon buttons (like the screenshot shows)
    panel
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::bottom(Val::Px(8.0)),
            column_gap: Val::Px(4.0),
            ..default()
        },))
        .with_children(|row| {
            // Navigation icons (simplified representations)
            for (icon, _button) in [
                ("🌍", SystemPanelButton::GotoPlanet),
                ("➡", SystemPanelButton::SendFleet),
                ("🔍", SystemPanelButton::SystemInfo),
                ("➤", SystemPanelButton::BuildShip),
            ] {
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(45.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_ICON_BG),
                    BorderColor::all(colors::PANEL_BORDER),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(icon),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::PANEL_TEXT),
                    ));
                });
            }
        });

    // Second row of navigation icons
    panel
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::bottom(Val::Px(12.0)),
            column_gap: Val::Px(4.0),
            ..default()
        },))
        .with_children(|row| {
            for icon in ["📍", "🚀", "⚙", "📋"] {
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(45.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_ICON_BG),
                    BorderColor::all(colors::PANEL_BORDER),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(icon),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::PANEL_TEXT),
                    ));
                });
            }
        });
}
