use bevy::prelude::*;

/// Types of icons that can be displayed in the info modal.
#[derive(Clone, Debug, Default)]
pub enum ModalIcon {
    /// Factory/industry building icon.
    Factory,
    /// Research/lab building icon.
    Laboratory,
    /// Shipyard/construction icon.
    Shipyard,
    /// Defense/military icon.
    Defense,
    /// Planet/colony icon.
    Planet,
    /// Ship icon.
    Ship,
    /// Research breakthrough icon.
    Research,
    /// Warning/alert icon.
    Warning,
    /// No icon.
    #[default]
    None,
}

/// Action for modal buttons.
#[derive(Clone, Debug)]
pub enum ModalAction {
    /// Close the modal.
    Dismiss,
    /// Navigate to a specific planet.
    GoToPlanet {
        system_index: usize,
        planet_index: usize,
    },
    /// Navigate to a specific star system.
    GoToSystem { system_index: usize },
    /// Open research screen.
    OpenResearch,
    /// Open ship designer.
    OpenShipDesign,
}

/// Configuration for a modal button.
#[derive(Clone, Debug)]
pub struct ModalButton {
    pub label: String,
    pub action: ModalAction,
}

/// State resource for the info modal dialog.
#[derive(Resource, Default)]
pub struct InfoModalState {
    /// Whether the modal is currently visible.
    pub visible: bool,
    /// Icon to display (optional).
    pub icon: ModalIcon,
    /// Main message text.
    pub message: String,
    /// Optional secondary/detail text.
    pub detail: Option<String>,
    /// Buttons to display.
    pub buttons: Vec<ModalButton>,
}

impl InfoModalState {
    /// Create a simple notification with just an OK button.
    pub fn notification(icon: ModalIcon, message: impl Into<String>) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail: None,
            buttons: vec![ModalButton {
                label: "OK".to_string(),
                action: ModalAction::Dismiss,
            }],
        }
    }

    /// Create a notification with a "Go to Planet" and "OK" buttons.
    pub fn planet_notification(
        icon: ModalIcon,
        message: impl Into<String>,
        system_index: usize,
        planet_index: usize,
    ) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail: None,
            buttons: vec![
                ModalButton {
                    label: "Go to Planet".to_string(),
                    action: ModalAction::GoToPlanet {
                        system_index,
                        planet_index,
                    },
                },
                ModalButton {
                    label: "OK".to_string(),
                    action: ModalAction::Dismiss,
                },
            ],
        }
    }

    /// Create a notification with custom buttons.
    pub fn custom(
        icon: ModalIcon,
        message: impl Into<String>,
        detail: Option<String>,
        buttons: Vec<ModalButton>,
    ) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail,
            buttons,
        }
    }

    /// Hide the modal.
    pub fn hide(&mut self) {
        self.visible = false;
    }
}

/// Marker for the modal overlay.
#[derive(Component)]
pub struct InfoModalOverlay;

/// Marker for modal buttons with their action.
#[derive(Component)]
pub struct InfoModalButton {
    pub action: ModalAction,
}
