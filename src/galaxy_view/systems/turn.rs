use bevy::prelude::*;

use crate::galaxy_view::modal::{InfoModalState, ModalAction, ModalButton, ModalIcon};
use crate::galaxy_view::types::GalaxyViewState;

/// Handle turn controls.
pub fn turn_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut map_state: ResMut<GalaxyViewState>,
    mut modal_state: ResMut<InfoModalState>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        map_state.turn_number += 1;
        info!("Turn {}", map_state.turn_number);

        // Demo: Show a notification every 5 turns
        if map_state.turn_number % 5 == 0 {
            *modal_state = InfoModalState::planet_notification(
                ModalIcon::Factory,
                format!(
                    "Factory construction complete on Terra Prime (Turn {})",
                    map_state.turn_number
                ),
                0,
                0,
            );
        }
    }

    // Press 'N' to show a test notification
    if keyboard.just_pressed(KeyCode::KeyN) {
        *modal_state = InfoModalState::planet_notification(
            ModalIcon::Factory,
            "Factory construction complete on Terra Prime",
            0,
            0,
        );
    }

    // Press 'M' to show a research notification
    if keyboard.just_pressed(KeyCode::KeyM) {
        *modal_state = InfoModalState::custom(
            ModalIcon::Research,
            "Research Complete: Advanced Propulsion",
            Some("Your scientists have discovered improved engine technology.".to_string()),
            vec![
                ModalButton {
                    label: "View Research".to_string(),
                    action: ModalAction::OpenResearch,
                },
                ModalButton {
                    label: "OK".to_string(),
                    action: ModalAction::Dismiss,
                },
            ],
        );
    }
}
