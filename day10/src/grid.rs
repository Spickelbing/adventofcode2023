use eyre::anyhow;

pub struct Grid {
    grid: Vec<Vec<Tile>>,
    dimensions: Dimensions,
    start_position: Coordinate,
}

pub struct Dimensions {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Grid {
    type Error = eyre::Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let grid: Vec<Vec<Tile>> = string
            .lines()
            .map(|line| line.chars().map(Tile::try_from).collect())
            .collect::<Result<_, _>>()?;

        let dimensions = Dimensions {
            x: if grid.len() == 0 { 0 } else { grid[0].len() },
            y: grid.len(),
        };

        let (start_x, start_y) = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, tile)| tile.is_start())
                    .map(|(x, _)| (x, y))
            })
            .ok_or(anyhow!("no start position"))?;

        if grid.iter().any(|row| row.len() != dimensions.x) {
            Err(anyhow!("invalid grid dimensions"))
        } else {
            Ok(Grid {
                grid,
                dimensions,
                start_position: Coordinate::new(start_x, start_y),
            })
        }
    }
}

impl Grid {
    pub fn get(&self, coordinate: Coordinate) -> Option<&Tile> {
        self.grid
            .get(coordinate.y)
            .map(|row| row.get(coordinate.x))?
    }

    pub fn start_position(&self) -> Coordinate {
        self.start_position
    }

    pub fn positions_of_traversable_pipes_from(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        if self.get(coordinate).is_none() {
            return Vec::new();
        }

        let tile = self.get(coordinate).unwrap();
        let mut traversable_pipes = Vec::new();

        if coordinate.y > 0 && tile.connects_north() {
            let northern_coordinate = Coordinate::new(coordinate.x, coordinate.y - 1);
            if self.get(northern_coordinate).unwrap().connects_south() {
                traversable_pipes.push(northern_coordinate);
            }
        }

        if coordinate.y < self.dimensions.y && tile.connects_south() {
            let southern_coordinate = Coordinate::new(coordinate.x, coordinate.y + 1);
            if self.get(southern_coordinate).unwrap().connects_north() {
                traversable_pipes.push(southern_coordinate);
            }
        }

        if coordinate.x > 0 && tile.connects_west() {
            let western_coordinate = Coordinate::new(coordinate.x - 1, coordinate.y);
            if self.get(western_coordinate).unwrap().connects_east() {
                traversable_pipes.push(western_coordinate)
            }
        }

        if coordinate.x < self.dimensions.x && tile.connects_east() {
            let eastern_coordinate = Coordinate::new(coordinate.x + 1, coordinate.y);
            if self.get(eastern_coordinate).unwrap().connects_west() {
                traversable_pipes.push(eastern_coordinate);
            }
        }

        traversable_pipes
    }
}

pub enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl Tile {
    fn is_start(&self) -> bool {
        match self {
            Tile::Start => true,
            _ => false,
        }
    }

    fn connects_north(&self) -> bool {
        match self {
            Tile::Start => true,
            Tile::Pipe(pipe) if pipe.connects_north() => true,
            _ => false,
        }
    }

    fn connects_south(&self) -> bool {
        match self {
            Tile::Start => true,
            Tile::Pipe(pipe) if pipe.connects_south() => true,
            _ => false,
        }
    }

    fn connects_east(&self) -> bool {
        match self {
            Tile::Start => true,
            Tile::Pipe(pipe) if pipe.connects_east() => true,
            _ => false,
        }
    }

    fn connects_west(&self) -> bool {
        match self {
            Tile::Start => true,
            Tile::Pipe(pipe) if pipe.connects_west() => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Pipe {
    fn connects_north(&self) -> bool {
        [Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest].contains(self)
    }

    fn connects_south(&self) -> bool {
        [Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest].contains(self)
    }

    fn connects_east(&self) -> bool {
        [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast].contains(self)
    }

    fn connects_west(&self) -> bool {
        [Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest].contains(self)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

impl TryFrom<char> for Tile {
    type Error = eyre::Error;
    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            '|' => Ok(Tile::Pipe(Pipe::Vertical)),
            '-' => Ok(Tile::Pipe(Pipe::Horizontal)),
            'L' => Ok(Tile::Pipe(Pipe::NorthEast)),
            'J' => Ok(Tile::Pipe(Pipe::NorthWest)),
            'F' => Ok(Tile::Pipe(Pipe::SouthEast)),
            '7' => Ok(Tile::Pipe(Pipe::SouthWest)),
            _ => Err(anyhow!("invalid character")),
        }
    }
}
