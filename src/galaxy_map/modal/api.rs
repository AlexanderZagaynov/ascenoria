use super::types::{InfoModalState, ModalIcon};

/// Public function to show a notification on the galaxy map.
pub fn show_notification(
    modal_state: &mut InfoModalState,
    icon: ModalIcon,
    message: impl Into<String>,
) {
    *modal_state = InfoModalState::notification(icon, message);
}

/// Public function to show a planet-related notification.
pub fn show_planet_notification(
    modal_state: &mut InfoModalState,
    icon: ModalIcon,
    message: impl Into<String>,
    system_index: usize,
    planet_index: usize,
) {
    *modal_state = InfoModalState::planet_notification(icon, message, system_index, planet_index);
}
