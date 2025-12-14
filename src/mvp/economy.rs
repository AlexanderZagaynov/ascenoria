use crate::mvp::{Building, Resources};

pub const BUILD_COST_INDUSTRY: i32 = 10;

pub fn apply_turn_production(
    resources: &mut Resources,
    buildings: impl Iterator<Item = Building>,
) -> (i32, i32, i32) {
    let mut food_income = 0;
    let mut industry_income = 0;
    let mut science_income = 0;
    let mut pop_income = 0;

    for building in buildings {
        if !building.is_constructed() {
            continue;
        }
        let yields = building.kind.yields();
        food_income += yields.food;
        industry_income += yields.industry;
        science_income += yields.science;
        pop_income += yields.pop_capacity;
    }

    resources.food += food_income;
    resources.industry += industry_income;
    resources.science += science_income;
    resources.pop_capacity += pop_income;

    (food_income, industry_income, science_income)
}

pub fn spend_industry_on_building(building: &mut Building, available_industry: &mut i32) {
    if building.is_constructed() {
        return;
    }
    if *available_industry <= 0 {
        return;
    }
    let spend = building.remaining_industry.min(*available_industry);
    building.remaining_industry -= spend;
    *available_industry -= spend;
}
