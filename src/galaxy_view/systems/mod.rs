mod camera;
mod interaction;
mod lifecycle;
mod turn;

pub use camera::galaxy_rotation_system;
pub use interaction::{panel_button_system, star_click_system};
pub use lifecycle::cleanup_galaxy_view;
pub use turn::turn_control_system;
