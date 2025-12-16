use bevy::prelude::*;

use crate::galaxy_view::colors;
use crate::galaxy_view::types::{GalaxyViewRoot, PanelButton};
use crate::galaxy_view::ui::{
    spawn_bottom_controls, spawn_panel_section, spawn_speed_controls, spawn_turn_indicators,
};

pub fn spawn_ui_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            GalaxyViewRoot,
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
                    // Turn indicators at top
                    spawn_turn_indicators(panel);

                    // Speed controls
                    spawn_speed_controls(panel);

                    // Main menu buttons
                    spawn_panel_section(panel, "Planets", PanelButton::Planets);
                    spawn_panel_section(panel, "Ships", PanelButton::Ships);
                    spawn_panel_section(panel, "Research", PanelButton::Research);
                    spawn_panel_section(panel, "Special Ability", PanelButton::SpecialAbility);
                    spawn_panel_section(panel, "Species", PanelButton::Species);

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

pub fn spawn_player_icon(commands: &mut Commands) {
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
            BorderColor::all(colors::RING_GREEN),
            GalaxyViewRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::RING_GREEN),
            ));
        });
}

pub fn spawn_instructions(commands: &mut Commands) {
    commands.spawn((
        Text::new("Rotate: LMB/RMB drag / Arrow keys / WASD • Zoom: Q/E • Reset: R • Click star to select, twice to enter"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::PANEL_TEXT_DIM),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        GalaxyViewRoot,
    ));
}
