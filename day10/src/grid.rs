use eyre::anyhow;

pub struct Grid {
    grid: Vec<Vec<Tile>>,
    dimensions: Dimensions,
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
            x: grid[0].len(),
            y: grid.len(),
        };

        if grid.iter().any(|row| row.len() != dimensions.x) {
            Err(anyhow!("invalid grid dimensions"))
        } else {
            Ok(Grid { grid, dimensions })
        }
    }
}

impl Grid {
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Tile>> {
        self.grid.iter().map(|x| x.iter())
    }

    pub fn get(&self, coordinate: Coordinate) -> Option<&Tile> {
        self.grid
            .get(coordinate.y)
            .map(|row| row.get(coordinate.x))?
    }

    pub fn positions_of_traversable_pipes_from(
        &self,
        pos_x: usize,
        pos_y: usize,
    ) -> Vec<Coordinate> {
        let mut traversable_pipes = Vec::new();

        if pos_y > 0 {
            let northern_coordinate = Coordinate {
                x: pos_x,
                y: pos_y - 1,
            };
            if let Some(Tile::Pipe(pipe)) = self.get(northern_coordinate) {
                if [Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest].contains(pipe) {
                    traversable_pipes.push(northern_coordinate);
                }
            }
        }

        if pos_y < self.dimensions.y {
            let southern_coordinate = Coordinate {
                x: pos_x,
                y: pos_y + 1,
            };
            if let Some(Tile::Pipe(pipe)) = self.get(southern_coordinate) {
                if [Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest].contains(pipe) {
                    traversable_pipes.push(southern_coordinate);
                }
            }
        }

        if pos_x > 0 {
            let western_coordinate = Coordinate {
                x: pos_x - 1,
                y: pos_y,
            };
            if let Some(Tile::Pipe(pipe)) = self.get(western_coordinate) {
                if [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast].contains(pipe) {
                    traversable_pipes.push(western_coordinate);
                }
            }
        }

        if pos_x < self.dimensions.x {
            let eastern_coordinate = Coordinate {
                x: pos_x + 1,
                y: pos_y,
            };
            if let Some(Tile::Pipe(pipe)) = self.get(eastern_coordinate) {
                if [Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest].contains(pipe) {
                    traversable_pipes.push(eastern_coordinate);
                }
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

#[derive(PartialEq, Eq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
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

impl Tile {
    pub fn is_start(&self) -> bool {
        match self {
            Tile::Start => true,
            _ => false,
        }
    }
}
