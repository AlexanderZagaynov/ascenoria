use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::game_options::types::ScrollButton;
use crate::game_options::ui::colors;

/// Spawns a scroll button (up or down).
pub fn spawn_scroll_button(
    col: &mut ChildSpawnerCommands,
    icon: &str,
    button_type: ScrollButton,
    is_up: bool,
) {
    col.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(24.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: if is_up {
                UiRect::bottom(Val::Px(5.0))
            } else {
                UiRect::top(Val::Px(5.0))
            },
            ..default()
        },
        BackgroundColor(colors::BUTTON_NORMAL),
        button_type,
    ))
    .with_children(|btn| {
        btn.spawn((
            Text::new(icon),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT),
        ));
    });
}
