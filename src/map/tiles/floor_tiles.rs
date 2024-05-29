/// Tile types in the floor layer.
#[derive(Clone, Copy, PartialEq)]
pub enum FloorTileType {
    Empty,
    Grass,
    Sand,
    Stone,
}

impl FloorTileType {
    pub fn supports_wall(&self) -> bool {
        match self {
            FloorTileType::Empty => false,
            _ => true,
        }
    }
}
