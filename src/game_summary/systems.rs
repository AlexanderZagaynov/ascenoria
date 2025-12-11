//! Systems for the game summary screen.

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::main_menu::GameState;

use super::types::{
    BackButton, ContinueButton, GameSummaryRoot, SummaryScrollContent, SummaryScrollViewport,
    colors,
};

/// Cleans up game summary entities.
pub fn cleanup_game_summary(mut commands: Commands, query: Query<Entity, With<GameSummaryRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handles button clicks for continue and back.
pub fn continue_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&ContinueButton>,
            Option<&BackButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color, is_continue, is_back) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(colors::BUTTON_PRESSED);
                if is_continue.is_some() {
                    info!("Continuing to game...");
                    next_state.set(GameState::InGame);
                } else if is_back.is_some() {
                    info!("Returning to game options...");
                    next_state.set(GameState::GameOptions);
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(colors::BUTTON_HOVER);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::BUTTON_NORMAL);
            }
        }
    }
}

/// Handles keyboard navigation (Enter to continue, Escape to go back).
pub fn keyboard_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Returning to game options...");
        next_state.set(GameState::GameOptions);
    } else if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        info!("Continuing to game...");
        next_state.set(GameState::InGame);
    }
}

/// Handles scrolling of the text content.
pub fn summary_scroll_system(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut viewport_query: Query<(&mut ScrollPosition, &ComputedNode), With<SummaryScrollViewport>>,
    content_query: Query<&ComputedNode, With<SummaryScrollContent>>,
) {
    let Some((mut scroll_pos, viewport_node)) = viewport_query.iter_mut().next() else {
        return;
    };
    let Some(content_node) = content_query.iter().next() else {
        return;
    };

    let viewport_height = viewport_node.size().y;
    let content_height = content_node.size().y;
    let max_scroll = (content_height - viewport_height).max(0.0);

    let mut delta = 0.0;
    for event in mouse_wheel_events.read() {
        delta -= event.y * 40.0;
    }

    if delta != 0.0 {
        scroll_pos.y = (scroll_pos.y + delta).clamp(0.0, max_scroll);
    }
}
