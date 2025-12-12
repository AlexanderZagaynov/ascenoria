use bevy::prelude::*;

use crate::planet_view::types::PlanetViewRoot;
use super::components::{PlanetInfoModalButton, PlanetInfoModalOverlay};
use super::state::PlanetInfoModalState;
use super::ui::spawn_modal_panel;

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
