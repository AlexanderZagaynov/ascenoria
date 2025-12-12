use crate::data::GameData;
use crate::game_options::types::{GameOptionsRoot, NewGameSettings};
use crate::game_options::ui;
use bevy::prelude::*;

pub fn setup_game_options(mut commands: Commands, game_data: Option<Res<GameData>>) {
    // Initialize settings if not present
    commands.init_resource::<NewGameSettings>();

    // Camera
    commands.spawn((Camera2d::default(), GameOptionsRoot));

    let species_list = game_data
        .as_ref()
        .map(|data| data.species().to_vec())
        .unwrap_or_default();

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(ui::colors::BACKGROUND),
            GameOptionsRoot,
        ))
        .with_children(|root| {
            // Left panel - Galaxy preview
            ui::spawn_galaxy_panel(root);

            // Center panel - Selected species info
            ui::spawn_species_info_panel(root, &species_list);

            // Right panel - Species list
            ui::spawn_species_list_panel(root, &species_list);
        });

    // Bottom bar with settings
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(120.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(ui::colors::PANEL_BG),
            GameOptionsRoot,
        ))
        .with_children(|bar| {
            ui::spawn_settings_buttons(bar);
            ui::spawn_begin_button(bar);
        });
}

pub fn cleanup_game_options(mut commands: Commands, query: Query<Entity, With<GameOptionsRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
