/// Types of tiles in the floor layer
#[derive(Clone, PartialEq)]
pub enum FloorTileType {
    Empty,
}

impl Default for FloorTileType {
    fn default() -> Self {
        FloorTileType::Empty
    }
}

/// Types of tiles in the wall layer
#[derive(Clone, PartialEq)]
pub enum WallTileType {
    Empty,
    Wall,
}

impl Default for WallTileType {
    fn default() -> Self {
        WallTileType::Empty
    }
}
