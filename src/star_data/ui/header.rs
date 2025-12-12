use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use super::super::types::colors;

pub fn spawn_system_header(panel: &mut ChildSpawnerCommands, system_name: &str) {
    panel
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::PANEL_BORDER),
        ))
        .with_children(|header| {
            header.spawn((
                Text::new(system_name),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::STAR_LABEL),
            ));

            header.spawn((
                Text::new("White Medium"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(colors::PANEL_TEXT_DIM),
            ));
        });
}
