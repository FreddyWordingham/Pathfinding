/// Types of tiles in the map
#[derive(Clone, PartialEq)]
pub enum TileType {
    Void,
    Floor,
    Wall,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Floor
    }
}
