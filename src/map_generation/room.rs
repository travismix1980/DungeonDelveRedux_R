#[derive(Debug, Clone)]
pub enum RoomType {
    Start,
    Empty,
    Wall,
    Monster,
    Chest,
    Boss,
    Exit,
}

pub struct DungeonMap {
    pub grid: Vec<Vec<RoomType>>,
}

impl DungeonMap {
    pub fn new() -> Self {
        use RoomType::*;
        let grid = vec![
            vec![Exit, Empty, Wall, Monster, Chest],
            vec![Empty, Wall, Empty, Empty, Empty],
            vec![Empty, Monster, Empty, Wall, Boss],
            vec![Empty, Empty, Chest, Empty, Empty],
            vec![Wall, Empty, Empty, Monster, Start],
        ];
        Self { grid }
    }

    pub fn get_room(&self, x: usize, y: usize) -> Option<&RoomType> {
        self.grid.get(y)?.get(x)
    }

    pub fn width(&self) -> usize {
        self.grid.first().map_or(0, |row| row.len())
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }
}