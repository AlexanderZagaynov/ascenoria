use bevy::prelude::*;
use crate::galaxy_map::colors;
use crate::galaxy_map::types::GalaxyMapRoot;
use super::types::{InfoModalButton, InfoModalOverlay, InfoModalState, ModalAction};
use super::ui::spawn_modal_panel;

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

/// Handle modal button clicks.
pub fn info_modal_button_system(
    mut interaction_query: Query<
        (&Interaction, &InfoModalButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut modal_state: ResMut<InfoModalState>,
    mut next_state: ResMut<NextState<crate::main_menu::GameState>>,
    mut star_state: ResMut<crate::star::StarState>,
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
                        star_index,
                        planet_index,
                    } => {
                        star_state.star_index = *star_index;
                        star_state.selected_planet = Some(*planet_index);
                        planet_view_state.planet_index = *planet_index;
                        modal_state.hide();
                        next_state.set(crate::main_menu::GameState::PlanetView);
                    }
                    ModalAction::GoToStar { star_index } => {
                        star_state.star_index = *star_index;
                        star_state.selected_planet = None;
                        modal_state.hide();
                        next_state.set(crate::main_menu::GameState::StarView);
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
