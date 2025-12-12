use bevy::prelude::*;
use crate::galaxy_view::colors;
use super::super::types::InfoModalState;
use super::sections::{spawn_modal_message_section, spawn_modal_top_section};
use super::controls::spawn_modal_button_row;

pub fn spawn_modal_panel(parent: &mut ChildSpawnerCommands, modal_state: &InfoModalState) {
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
