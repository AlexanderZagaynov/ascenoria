use crate::game_options::types::*;
use bevy::{ecs::message::MessageReader, input::mouse::MouseWheel, prelude::*};

/// Handles scrolling of the species list.
pub fn species_scroll_system(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut viewport_query: Query<
        (&mut ScrollPosition, &ComputedNode, &Children),
        With<SpeciesListViewport>,
    >,
    mut thumb_query: Query<
        &mut Node,
        (
            With<SpeciesListScrollThumb>,
            Without<SpeciesListViewport>,
            Without<SpeciesListItem>,
        ),
    >,
    button_query: Query<(&Interaction, &ScrollButton), (Changed<Interaction>, With<Button>)>,
    item_query: Query<
        (&ComputedNode, &Node),
        (
            With<SpeciesListItem>,
            Without<SpeciesListViewport>,
            Without<SpeciesListScrollThumb>,
        ),
    >,
) {
    let Some((mut scroll_pos, viewport_computed, children)) = viewport_query.iter_mut().next()
    else {
        return;
    };
    let Some(mut thumb_node) = thumb_query.iter_mut().next() else {
        return;
    };

    // Get visible height from viewport
    let visible_height = viewport_computed.size().y;

    // Calculate item height dynamically
    let item_height = if let Some(first_child) = children.first() {
        if let Ok((computed, style)) = item_query.get(*first_child) {
            let h = computed.size().y;
            let margin = match style.margin.bottom {
                Val::Px(v) => v,
                _ => 0.0,
            };
            let total = h + margin;
            if h > 0.0 { total } else { 85.0 }
        } else {
            85.0
        }
    } else {
        85.0
    };

    let total_items = children.len() as f32;
    let total_height = total_items * item_height;
    let max_scroll = (total_height - visible_height).max(0.0);

    let current_top = scroll_pos.y;
    let mut new_top = current_top;

    // Mouse Wheel
    for event in mouse_wheel_events.read() {
        new_top -= event.y * 40.0;
    }

    // Buttons
    for (interaction, button) in &button_query {
        if *interaction == Interaction::Pressed {
            match button {
                ScrollButton::Up => new_top -= 40.0,
                ScrollButton::Down => new_top += 40.0,
            }
        }
    }

    // Clamp
    new_top = new_top.clamp(0.0, max_scroll);

    // Apply
    scroll_pos.y = new_top;

    // Update Thumb
    if total_height > 0.0 {
        let viewport_ratio = (visible_height / total_height).clamp(0.1, 1.0); // Min 10% thumb size
        let thumb_height_percent = viewport_ratio * 100.0;
        thumb_node.height = Val::Percent(thumb_height_percent);

        if max_scroll > 0.0 {
            let scroll_percent = new_top / max_scroll;
            // The track is 100%. The thumb takes up `thumb_height_percent`.
            // The available travel space is 100% - thumb_height_percent.
            let available_travel_percent = 100.0 - thumb_height_percent;
            let thumb_top_percent = scroll_percent * available_travel_percent;
            thumb_node.top = Val::Percent(thumb_top_percent);
        } else {
            thumb_node.top = Val::Percent(0.0);
        }
    }
}
