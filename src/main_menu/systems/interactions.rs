use bevy::{ecs::message::MessageWriter, prelude::*};

use crate::main_menu::GameState;
use crate::main_menu::colors;
use crate::main_menu::components::MenuButton;

/// Handles button interaction visual feedback.
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(colors::BUTTON_PRESSED);
                *border_color = BorderColor::all(colors::BUTTON_TEXT);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(colors::BUTTON_HOVERED);
                *border_color = BorderColor::all(colors::BUTTON_TEXT.with_alpha(0.8));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::BUTTON_NORMAL);
                *border_color = BorderColor::all(colors::BUTTON_BORDER);
            }
        }
    }
}

/// Handles menu button actions.
pub fn menu_action_system(
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit_events: MessageWriter<AppExit>,
) {
    // Keyboard shortcuts
    let alt_pressed = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);

    if alt_pressed {
        if keyboard.just_pressed(KeyCode::KeyX) {
            exit_events.write(AppExit::Success);
        }
    }

    // Button clicks
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::NewGame => {
                    info!("Starting new game...");
                    next_state.set(GameState::PlanetView);
                }
                MenuButton::Exit => {
                    exit_events.write(AppExit::Success);
                }
            }
        }
    }
}
