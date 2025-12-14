#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn neighbors_4(self) -> [GridPos; 4] {
        [
            GridPos {
                x: self.x - 1,
                y: self.y,
            },
            GridPos {
                x: self.x + 1,
                y: self.y,
            },
            GridPos {
                x: self.x,
                y: self.y - 1,
            },
            GridPos {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Plains,
    Forest,
    Hills,
    Desert,
    Void, // “black”, connector-only
}

impl CellType {
    pub fn is_void(self) -> bool {
        matches!(self, CellType::Void)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingKind {
    Housing,
    Food,
    Industry,
    Science,
    Connector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildingYields {
    pub food: i32,
    pub industry: i32,
    pub science: i32,
    pub pop_capacity: i32,
}

impl BuildingKind {
    pub fn yields(self) -> BuildingYields {
        match self {
            BuildingKind::Housing => BuildingYields {
                food: 0,
                industry: 0,
                science: 0,
                pop_capacity: 1,
            },
            BuildingKind::Food => BuildingYields {
                food: 1,
                industry: 0,
                science: 0,
                pop_capacity: 0,
            },
            BuildingKind::Industry => BuildingYields {
                food: 0,
                industry: 1,
                science: 0,
                pop_capacity: 0,
            },
            BuildingKind::Science => BuildingYields {
                food: 0,
                industry: 0,
                science: 1,
                pop_capacity: 0,
            },
            BuildingKind::Connector => BuildingYields {
                food: 0,
                industry: 0,
                science: 0,
                pop_capacity: 0,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Building {
    pub kind: BuildingKind,
    pub remaining_industry: i32,
}

impl Building {
    pub fn is_constructed(self) -> bool {
        self.remaining_industry <= 0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Resources {
    pub food: i32,
    pub industry: i32,
    pub science: i32,
    pub pop_capacity: i32,
}
