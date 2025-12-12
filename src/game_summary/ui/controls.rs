use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use super::super::types::{BackButton, ContinueButton, colors};

/// Spawns the bottom button area.
pub fn spawn_button_area(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        })
        .with_children(|button_area| {
            // Back button
            spawn_button(button_area, "← Back to Options", BackButton);

            // Continue button
            spawn_button(button_area, "Continue", ContinueButton);
        });
}

/// Spawns a button with the given label and marker component.
fn spawn_button<M: Component>(button_area: &mut ChildSpawnerCommands, label: &str, marker: M) {
    button_area
        .spawn((
            Button,
            Node {
                padding: UiRect::new(Val::Px(30.0), Val::Px(30.0), Val::Px(15.0), Val::Px(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(colors::BUTTON_NORMAL),
            BorderColor::all(colors::PANEL_BORDER),
            BorderRadius::all(Val::Px(6.0)),
            marker,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT),
            ));
        });
}
