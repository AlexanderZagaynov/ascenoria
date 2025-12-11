//! Modal dialog system for the galaxy map.
//!
//! Provides popup notifications for events like construction complete,
//! research done, etc. Supports custom icons, messages, and action buttons.

use bevy::prelude::*;

use super::colors;
use super::types::GalaxyMapRoot;

/// Types of icons that can be displayed in the info modal.
#[derive(Clone, Debug, Default)]
pub enum ModalIcon {
    /// Factory/industry building icon.
    Factory,
    /// Research/lab building icon.
    Laboratory,
    /// Shipyard/construction icon.
    Shipyard,
    /// Defense/military icon.
    Defense,
    /// Planet/colony icon.
    Planet,
    /// Ship icon.
    Ship,
    /// Research breakthrough icon.
    Research,
    /// Warning/alert icon.
    Warning,
    /// No icon.
    #[default]
    None,
}

/// Action for modal buttons.
#[derive(Clone, Debug)]
pub enum ModalAction {
    /// Close the modal.
    Dismiss,
    /// Navigate to a specific planet.
    GoToPlanet {
        system_index: usize,
        planet_index: usize,
    },
    /// Navigate to a specific star system.
    GoToSystem { system_index: usize },
    /// Open research screen.
    OpenResearch,
    /// Open ship designer.
    OpenShipDesign,
}

/// Configuration for a modal button.
#[derive(Clone, Debug)]
pub struct ModalButton {
    pub label: String,
    pub action: ModalAction,
}

/// State resource for the info modal dialog.
#[derive(Resource, Default)]
pub struct InfoModalState {
    /// Whether the modal is currently visible.
    pub visible: bool,
    /// Icon to display (optional).
    pub icon: ModalIcon,
    /// Main message text.
    pub message: String,
    /// Optional secondary/detail text.
    pub detail: Option<String>,
    /// Buttons to display.
    pub buttons: Vec<ModalButton>,
}

impl InfoModalState {
    /// Create a simple notification with just an OK button.
    pub fn notification(icon: ModalIcon, message: impl Into<String>) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail: None,
            buttons: vec![ModalButton {
                label: "OK".to_string(),
                action: ModalAction::Dismiss,
            }],
        }
    }

    /// Create a notification with a "Go to Planet" and "OK" buttons.
    pub fn planet_notification(
        icon: ModalIcon,
        message: impl Into<String>,
        system_index: usize,
        planet_index: usize,
    ) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail: None,
            buttons: vec![
                ModalButton {
                    label: "Go to Planet".to_string(),
                    action: ModalAction::GoToPlanet {
                        system_index,
                        planet_index,
                    },
                },
                ModalButton {
                    label: "OK".to_string(),
                    action: ModalAction::Dismiss,
                },
            ],
        }
    }

    /// Create a notification with custom buttons.
    pub fn custom(
        icon: ModalIcon,
        message: impl Into<String>,
        detail: Option<String>,
        buttons: Vec<ModalButton>,
    ) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail,
            buttons,
        }
    }

    /// Hide the modal.
    pub fn hide(&mut self) {
        self.visible = false;
    }
}

/// Marker for the modal overlay.
#[derive(Component)]
pub struct InfoModalOverlay;

/// Marker for modal buttons with their action.
#[derive(Component)]
pub struct InfoModalButton {
    pub action: ModalAction,
}

/// Spawn the info modal when visible.
pub fn info_modal_system(
    mut commands: Commands,
    modal_state: Res<InfoModalState>,
    modal_query: Query<Entity, With<InfoModalOverlay>>,
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

    // Spawn the modal
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            GlobalZIndex(100), // Above all other UI
            InfoModalOverlay,
            GalaxyMapRoot,
        ))
        .with_children(|parent| {
            spawn_modal_panel(parent, &modal_state);
        });
}

fn spawn_modal_panel(parent: &mut ChildSpawnerCommands, modal_state: &InfoModalState) {
    parent
        .spawn((
            Node {
                width: Val::Px(420.0),
                min_height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(0.0)),
                ..default()
            },
            BackgroundColor(colors::PANEL_BG),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|panel| {
            spawn_modal_top_section(panel, &modal_state.icon);
            spawn_modal_message_section(panel, &modal_state.message, &modal_state.detail);
            spawn_modal_button_row(panel, &modal_state.buttons);
        });
}

fn spawn_modal_top_section(panel: &mut ChildSpawnerCommands, icon: &ModalIcon) {
    panel
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(30.0),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderRadius::top(Val::Px(8.0)),
        ))
        .with_children(|top| {
            let icon_text = match icon {
                ModalIcon::Factory => "🏭",
                ModalIcon::Laboratory => "🔬",
                ModalIcon::Shipyard => "🚀",
                ModalIcon::Defense => "🛡️",
                ModalIcon::Planet => "🌍",
                ModalIcon::Ship => "🛸",
                ModalIcon::Research => "💡",
                ModalIcon::Warning => "⚠️",
                ModalIcon::None => "",
            };

            if !icon_text.is_empty() {
                // Icon container
                top.spawn((
                    Node {
                        width: Val::Px(70.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.22, 0.25)),
                    BorderRadius::all(Val::Px(6.0)),
                ))
                .with_children(|icon_box| {
                    icon_box.spawn((
                        Text::new(icon_text),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

                // Planet preview circle
                top.spawn((
                    Node {
                        width: Val::Px(70.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.7)),
                    BorderRadius::all(Val::Px(35.0)),
                ))
                .with_children(|planet_preview| {
                    planet_preview.spawn((
                        Node {
                            width: Val::Px(60.0),
                            height: Val::Px(60.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.6, 0.3)),
                        BorderRadius::all(Val::Px(30.0)),
                    ));
                });
            }
        });
}

fn spawn_modal_message_section(
    panel: &mut ChildSpawnerCommands,
    message: &str,
    detail: &Option<String>,
) {
    panel
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::new(Val::Px(25.0), Val::Px(25.0), Val::Px(20.0), Val::Px(10.0)),
            ..default()
        })
        .with_children(|msg_section| {
            msg_section.spawn((
                Text::new(message),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::PANEL_TEXT),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    max_width: Val::Px(350.0),
                    ..default()
                },
            ));

            if let Some(detail_text) = detail {
                msg_section.spawn((
                    Text::new(detail_text),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::PANEL_TEXT_DIM),
                    TextLayout::new_with_justify(Justify::Center),
                    Node {
                        margin: UiRect::top(Val::Px(8.0)),
                        max_width: Val::Px(350.0),
                        ..default()
                    },
                ));
            }
        });
}

fn spawn_modal_button_row(panel: &mut ChildSpawnerCommands, buttons: &[ModalButton]) {
    panel
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(15.0),
            padding: UiRect::new(Val::Px(25.0), Val::Px(25.0), Val::Px(10.0), Val::Px(20.0)),
            ..default()
        })
        .with_children(|button_row| {
            for modal_button in buttons {
                let is_primary = !matches!(modal_button.action, ModalAction::Dismiss);
                let bg_color = if is_primary {
                    Color::srgb(0.2, 0.5, 0.6)
                } else {
                    colors::PANEL_DARK
                };

                button_row
                    .spawn((
                        Button,
                        Node {
                            padding: UiRect::new(
                                Val::Px(20.0),
                                Val::Px(20.0),
                                Val::Px(10.0),
                                Val::Px(10.0),
                            ),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(bg_color),
                        BorderRadius::all(Val::Px(4.0)),
                        InfoModalButton {
                            action: modal_button.action.clone(),
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(&modal_button.label),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(colors::PANEL_TEXT),
                        ));
                    });
            }
        });
}

/// Handle modal button clicks.
pub fn info_modal_button_system(
    mut interaction_query: Query<
        (&Interaction, &InfoModalButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut modal_state: ResMut<InfoModalState>,
    mut next_state: ResMut<NextState<crate::main_menu::GameState>>,
    mut star_system_state: ResMut<crate::star_system::StarSystemState>,
    mut planet_view_state: ResMut<crate::planet_view::PlanetViewState>,
) {
    for (interaction, modal_button, mut bg_color) in &mut interaction_query {
        let is_primary = !matches!(modal_button.action, ModalAction::Dismiss);
        let base_color = if is_primary {
            Color::srgb(0.2, 0.5, 0.6)
        } else {
            colors::PANEL_DARK
        };

        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(base_color.with_alpha(0.8));

                match &modal_button.action {
                    ModalAction::Dismiss => {
                        modal_state.hide();
                    }
                    ModalAction::GoToPlanet {
                        system_index,
                        planet_index,
                    } => {
                        star_system_state.system_index = *system_index;
                        star_system_state.selected_planet = Some(*planet_index);
                        planet_view_state.planet_index = *planet_index;
                        modal_state.hide();
                        next_state.set(crate::main_menu::GameState::PlanetView);
                    }
                    ModalAction::GoToSystem { system_index } => {
                        star_system_state.system_index = *system_index;
                        star_system_state.selected_planet = None;
                        modal_state.hide();
                        next_state.set(crate::main_menu::GameState::StarSystem);
                    }
                    ModalAction::OpenResearch => {
                        modal_state.hide();
                        info!("Open research (not yet implemented)");
                    }
                    ModalAction::OpenShipDesign => {
                        modal_state.hide();
                        info!("Open ship design (not yet implemented)");
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(base_color.lighter(0.1));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(base_color);
            }
        }
    }
}

/// Public function to show a notification on the galaxy map.
pub fn show_notification(
    modal_state: &mut InfoModalState,
    icon: ModalIcon,
    message: impl Into<String>,
) {
    *modal_state = InfoModalState::notification(icon, message);
}

/// Public function to show a planet-related notification.
pub fn show_planet_notification(
    modal_state: &mut InfoModalState,
    icon: ModalIcon,
    message: impl Into<String>,
    system_index: usize,
    planet_index: usize,
) {
    *modal_state = InfoModalState::planet_notification(icon, message, system_index, planet_index);
}
