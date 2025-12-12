#[cfg(test)]
mod tests {
    use crate::data_types::{load_game_data, GameData, GameRegistry};
    use crate::ship_design::{DesignError, ModuleCategory, ShipDesign};
    use std::path::PathBuf;

    fn load_assets() -> (GameData, GameRegistry) {
        load_game_data(PathBuf::from("assets/data")).expect("assets should load")
    }

    fn engine_id(data: &GameData) -> &str {
        data.engines()
            .first()
            .map(|e| e.id.as_str())
            .expect("engine in assets")
    }

    #[test]
    fn validates_design_within_limits() {
        let (data, registry) = load_assets();
        let hull_id = "small";
        let engine = engine_id(&data);

        let mut design = ShipDesign::new(hull_id);
        design.add_module(ModuleCategory::Engine, engine);

        assert!(design.validate(&data, &registry).is_ok());
    }

    #[test]
    fn rejects_design_without_engine() {
        let (data, registry) = load_assets();
        let hull_id = "small";

        let mut design = ShipDesign::new(hull_id);
        design.add_module(ModuleCategory::Weapon, "laser_beam");

        let err = design
            .validate(&data, &registry)
            .expect_err("missing engine should fail");
        assert!(matches!(err, DesignError::MissingEngine));
    }

    #[test]
    fn rejects_over_capacity_design() {
        let (data, registry) = load_assets();
        let hull_id = "small";
        let engine = engine_id(&data);

        let mut design = ShipDesign::new(hull_id);
        for _ in 0..6 {
            design.add_module(ModuleCategory::Engine, engine);
        }

        let err = design
            .validate(&data, &registry)
            .expect_err("over capacity should fail");
        assert!(matches!(err, DesignError::TooManyModules { .. }));
    }

    #[test]
    fn rejects_unknown_module_id() {
        let (data, registry) = load_assets();
        let hull_id = "small";

        let mut design = ShipDesign::new(hull_id);
        design.add_module(ModuleCategory::Engine, "unknown_engine");

        let err = design
            .validate(&data, &registry)
            .expect_err("unknown module should fail");
        assert!(matches!(
            err,
            DesignError::ModuleNotFound {
                category: ModuleCategory::Engine,
                ..
            }
        ));
    }
}
