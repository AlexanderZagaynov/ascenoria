use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use crate::galaxy_data::StarSystem;
use super::super::types::{PlanetVisual, colors};

pub fn spawn_planet_info_area(panel: &mut ChildSpawnerCommands, system: Option<&StarSystem>) {
    // Planet preview area (large image-like box)
    panel
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.08, 0.12)),
            BorderColor::all(colors::SELECTION_GREEN),
        ))
        .with_children(|preview| {
            // Planet name label
            if let Some(sys) = system {
                if let Some(planet) = sys.planets.first() {
                    preview.spawn((
                        Text::new(format!("{} I", sys.name.replace("System-", "Icarus"))),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::PLANET_LABEL),
                    ));

                    // Placeholder planet representation
                    preview.spawn((
                        Node {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(
                            PlanetVisual::from_surface_type(&planet.surface_type_id)
                                .primary_color(),
                        ),
                        BorderRadius::all(Val::Percent(50.0)),
                    ));
                }
            }
        });
}
