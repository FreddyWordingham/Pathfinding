/// Tile types in the floor layer.
#[derive(Clone, PartialEq)]
pub enum FloorTileType {
    Empty,
    Grass,
    Sand,
    Stone,
}

impl Default for FloorTileType {
    fn default() -> Self {
        FloorTileType::Empty
    }
}
