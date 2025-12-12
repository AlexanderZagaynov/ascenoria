use bevy::prelude::*;

/// State for the planet info modal overlay.
#[derive(Resource, Default)]
pub struct PlanetInfoModalState {
    /// Whether the modal is visible.
    pub visible: bool,
    /// Planet name to display.
    pub planet_name: String,
    /// Prosperity rate per day.
    pub prosperity_per_day: i32,
    /// Days until next population growth.
    pub days_to_growth: i32,
    /// Current population count.
    pub population: i32,
    /// Maximum population capacity.
    pub max_population: i32,
}

impl PlanetInfoModalState {
    /// Show the modal with planet info.
    pub fn show(
        &mut self,
        name: impl Into<String>,
        prosperity: i32,
        days: i32,
        pop: i32,
        max_pop: i32,
    ) {
        self.visible = true;
        self.planet_name = name.into();
        self.prosperity_per_day = prosperity;
        self.days_to_growth = days;
        self.population = pop;
        self.max_population = max_pop;
    }

    /// Hide the modal.
    pub fn hide(&mut self) {
        self.visible = false;
    }
}
