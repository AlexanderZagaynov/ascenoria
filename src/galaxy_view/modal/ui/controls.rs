use bevy::prelude::*;
use crate::galaxy_view::colors;
use super::super::types::{InfoModalButton, ModalAction, ModalButton};

pub fn spawn_modal_button_row(panel: &mut ChildSpawnerCommands, buttons: &[ModalButton]) {
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
