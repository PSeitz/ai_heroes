

#[derive(Debug, Default)]
pub struct Map {
    pub world: Vec<Vec<Cell>>,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32,) -> Self {
        let mut map = Map::default();
        map.width = width;
        map.height = height;
        map.world = (0..width).map(|_|{
            (0..height).map(|_|Cell{terrain_type:TerrainType::default()}).collect()
        }).collect();
        map
    }

}


#[derive(Debug)]
pub struct Cell {
    pub terrain_type: TerrainType
}

#[derive(Debug)]
pub enum TerrainType {
    Road,
    Grass,
    Dirt,
    Wood,
    Mountain,
}

impl Default for TerrainType {
    fn default() -> Self{
        TerrainType::Grass
    }
}
