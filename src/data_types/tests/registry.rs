use super::helpers::base_game_data;
use crate::data_types::entities::SurfaceCellType;
use crate::data_types::registry::GameRegistry;

#[test]
fn rejects_duplicate_ids() {
    let mut data = base_game_data();
    data.surface_cell_types = vec![
        SurfaceCellType {
            id: "duplicate".to_string(),
            name_en: "Duplicate".to_string(),
            is_usable: true,
        },
        SurfaceCellType {
            id: "duplicate".to_string(),
            name_en: "Duplicate Two".to_string(),
            is_usable: false,
        },
    ];

    let error = GameRegistry::from_game_data(&data).expect_err("Duplicate ids should be reported");

    match error {
        crate::data_types::errors::DataLoadError::DuplicateId { kind, id } => {
            assert_eq!(kind, "surface_cell_type");
            assert_eq!(id, "duplicate");
        }
        other => panic!("Unexpected error: {other:?}"),
    }
}
