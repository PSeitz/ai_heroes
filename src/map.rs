

#[derive(Debug, Default)]
struct Map {
    world: Vec<Cell>,
    width: u32,
    height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32,) -> Self {
        let mut map = Map::default();
        for _ in 0..(width*height) {
            map.world.push(Cell{terrain_type:TerrainType::default()});
        }
        map
    }
}


#[derive(Debug)]
struct Cell {
    terrain_type: TerrainType
}

#[derive(Debug)]
enum TerrainType {
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
