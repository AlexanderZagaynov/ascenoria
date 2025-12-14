use crate::mvp::{BuildingKind, CellType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceError {
    Occupied,
    NotAdjacent,
    ConnectorMustBeOnVoid,
    NonConnectorCannotBeOnVoid,
}

pub fn can_place_building(
    building_kind: BuildingKind,
    cell_type: CellType,
    cell_is_empty: bool,
    any_buildings_exist: bool,
    has_orthogonal_neighbor_building: bool,
) -> Result<(), PlaceError> {
    if !cell_is_empty {
        return Err(PlaceError::Occupied);
    }

    match (cell_type.is_void(), building_kind) {
        (true, BuildingKind::Connector) => {}
        (true, _) => return Err(PlaceError::NonConnectorCannotBeOnVoid),
        (false, BuildingKind::Connector) => return Err(PlaceError::ConnectorMustBeOnVoid),
        (false, _) => {}
    }

    if !any_buildings_exist {
        return Ok(());
    }

    if has_orthogonal_neighbor_building {
        Ok(())
    } else {
        Err(PlaceError::NotAdjacent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mvp::{BuildingKind, CellType};

    #[test]
    fn first_building_can_be_placed_anywhere_on_usable_cell() {
        let result =
            can_place_building(BuildingKind::Housing, CellType::Plains, true, false, false);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn subsequent_building_requires_adjacency() {
        let result = can_place_building(BuildingKind::Food, CellType::Plains, true, true, false);
        assert_eq!(result, Err(PlaceError::NotAdjacent));

        let result = can_place_building(BuildingKind::Food, CellType::Plains, true, true, true);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn connector_only_on_void_and_others_not_on_void() {
        assert_eq!(
            can_place_building(BuildingKind::Connector, CellType::Void, true, false, false),
            Ok(())
        );
        assert_eq!(
            can_place_building(
                BuildingKind::Connector,
                CellType::Plains,
                true,
                false,
                false
            ),
            Err(PlaceError::ConnectorMustBeOnVoid)
        );
        assert_eq!(
            can_place_building(BuildingKind::Housing, CellType::Void, true, false, false),
            Err(PlaceError::NonConnectorCannotBeOnVoid)
        );
    }
}
