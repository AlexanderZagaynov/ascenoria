use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use super::super::types::colors;

/// Spawns the title section with species name.
pub fn spawn_title_section(panel: &mut ChildSpawnerCommands, species_name: &str) {
    panel
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::bottom(Val::Px(10.0)),
            border: UiRect::bottom(Val::Px(1.0)),
            ..default()
        })
        .with_children(|title_area| {
            // Species name (large title)
            title_area.spawn((
                Text::new(species_name),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(colors::TITLE),
            ));

            // Subtitle
            title_area.spawn((
                Text::new("Game Summary"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::SUBTITLE),
                Node {
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
            ));
        });
}
