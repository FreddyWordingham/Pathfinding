/// Tile types in the floor layer.
#[derive(Clone, Copy, PartialEq)]
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
