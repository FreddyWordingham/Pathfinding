/// Tile types in the wall layer.
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
