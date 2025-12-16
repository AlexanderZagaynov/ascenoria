use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::GalaxyPreview;

use super::super::rendering::get_planet_thumbnail_color;
use super::super::types::{PanelButton, PlanetThumbnail, colors};

/// Spawn the top navigation bar with planet thumbnails.
pub fn spawn_top_bar(
    root: &mut ChildSpawnerCommands,
    num_planets: usize,
    planet_index: usize,
    star_index: usize,
    galaxy_preview: &GalaxyPreview,
    planet_name: &str,
    surface_type: &str,
    planet_size: &str,
) {
    root.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(80.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.0)),
            column_gap: Val::Px(8.0),
            border: UiRect::bottom(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(colors::PANEL_BG.with_alpha(0.9)),
        BorderColor::all(colors::BORDER),
    ))
    .with_children(|top_bar| {
        // Left section: Back button
        spawn_back_button(top_bar);

        // Center section: Planet info
        spawn_planet_info(top_bar, planet_name, surface_type, planet_size);

        // Right section: Planet thumbnails
        spawn_planet_thumbnails(
            top_bar,
            num_planets,
            planet_index,
            star_index,
            galaxy_preview,
        );
    });
}

/// Spawn the back button.
fn spawn_back_button(top_bar: &mut ChildSpawnerCommands) {
    top_bar
        .spawn((
            Button,
            Node {
                width: Val::Px(60.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(colors::BUTTON_NORMAL),
            BorderColor::all(colors::BORDER),
            PanelButton::Back,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("◀"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::HEADER_TEXT),
            ));
        });
}

/// Spawn the planet info section.
fn spawn_planet_info(
    top_bar: &mut ChildSpawnerCommands,
    planet_name: &str,
    surface_type: &str,
    planet_size: &str,
) {
    top_bar
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|info| {
            info.spawn((
                Text::new(planet_name),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::HEADER_TEXT),
            ));
            info.spawn((
                Text::new(format!("{} • {}", surface_type, planet_size)),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
            ));
        });
}

/// Spawn the planet thumbnail buttons.
fn spawn_planet_thumbnails(
    top_bar: &mut ChildSpawnerCommands,
    num_planets: usize,
    planet_index: usize,
    star_index: usize,
    galaxy_preview: &GalaxyPreview,
) {
    top_bar
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|thumbs| {
            for i in 0..num_planets {
                let is_selected = i == planet_index;
                let border_color = if is_selected {
                    colors::THUMBNAIL_SELECTED
                } else {
                    colors::THUMBNAIL_NORMAL
                };

                let thumb_color = galaxy_preview
                    .galaxy
                    .systems
                    .get(star_index)
                    .and_then(|s| s.planets.get(i))
                    .map(|p| get_planet_thumbnail_color(&p.surface_type_id))
                    .unwrap_or(colors::TILE_WHITE);

                thumbs
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(thumb_color),
                        BorderColor::all(border_color),
                        PlanetThumbnail(i),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(format!("{}", i + 1)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}
