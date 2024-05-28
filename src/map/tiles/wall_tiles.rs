/// Tile types in the wall layer.
#[derive(Clone, Copy, PartialEq)]
pub enum WallTileType {
    Empty,
    Wall,
}

impl Default for WallTileType {
    fn default() -> Self {
        WallTileType::Empty
    }
}
