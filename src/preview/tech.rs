use bevy::prelude::*;
use std::collections::HashSet;
use crate::data::{Tech, NO_TECH_REQUIREMENT};

/// Tracks unlocked technologies by index for filtering build options.
#[derive(Resource, Default)]
pub struct TechState {
    pub completed: HashSet<String>,
}

impl TechState {
    pub fn is_unlocked(&self, tech_index: i32, techs: &[Tech]) -> bool {
        if tech_index == NO_TECH_REQUIREMENT {
            return true;
        }
        techs
            .get(tech_index as usize)
            .map(|t| self.completed.contains(&t.id))
            .unwrap_or(false)
    }
}
