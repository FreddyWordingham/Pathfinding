/// Tile types in the wall layer.
#[derive(Clone, Copy, PartialEq)]
pub enum WallTileType {
    Empty,
    Wall,
}

impl WallTileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            WallTileType::Empty => true,
            WallTileType::Wall => false,
        }
    }
}
