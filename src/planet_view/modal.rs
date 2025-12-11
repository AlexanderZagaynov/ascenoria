//! Planet info modal dialog system.
//!
//! Displays planet prosperity information and population growth stats.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use super::types::{PlanetViewRoot, colors};

/// State for the planet info modal overlay.
#[derive(Resource, Default)]
pub struct PlanetInfoModalState {
    /// Whether the modal is visible.
    pub visible: bool,
    /// Planet name to display.
    pub planet_name: String,
    /// Prosperity rate per day.
    pub prosperity_per_day: i32,
    /// Days until next population growth.
    pub days_to_growth: i32,
    /// Current population count.
    pub population: i32,
    /// Maximum population capacity.
    pub max_population: i32,
}

impl PlanetInfoModalState {
    /// Show the modal with planet info.
    pub fn show(
        &mut self,
        name: impl Into<String>,
        prosperity: i32,
        days: i32,
        pop: i32,
        max_pop: i32,
    ) {
        self.visible = true;
        self.planet_name = name.into();
        self.prosperity_per_day = prosperity;
        self.days_to_growth = days;
        self.population = pop;
        self.max_population = max_pop;
    }

    /// Hide the modal.
    pub fn hide(&mut self) {
        self.visible = false;
    }
}

/// Marker for the planet info modal overlay.
#[derive(Component)]
pub struct PlanetInfoModalOverlay;

/// Marker for the modal OK button.
#[derive(Component)]
pub struct PlanetInfoModalButton;

/// Spawn or despawn the planet info modal based on state.
pub fn planet_info_modal_system(
    mut commands: Commands,
    modal_state: Res<PlanetInfoModalState>,
    modal_query: Query<Entity, With<PlanetInfoModalOverlay>>,
) {
    // Only process if state changed
    if !modal_state.is_changed() {
        return;
    }

    // Despawn existing modal if any
    for entity in modal_query.iter() {
        commands.entity(entity).despawn();
    }

    // If not visible, we're done
    if !modal_state.visible {
        return;
    }

    // Spawn the modal overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            GlobalZIndex(100),
            PlanetInfoModalOverlay,
            PlanetViewRoot,
        ))
        .with_children(|parent| {
            spawn_modal_panel(parent, &modal_state);
        });
}

/// Spawn the modal panel content.
fn spawn_modal_panel(parent: &mut ChildSpawnerCommands, modal_state: &PlanetInfoModalState) {
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

/// Handle modal button clicks.
pub fn planet_info_modal_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlanetInfoModalButton>),
    >,
    mut modal_state: ResMut<PlanetInfoModalState>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                modal_state.hide();
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.28, 0.35));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.15, 0.2, 0.25));
            }
        }
    }
}
