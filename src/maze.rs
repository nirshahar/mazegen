use std::fmt::Display;

use rand::seq::SliceRandom;

use grid::Grid;

use crate::union_find::{FastUnionFind, UnionFind, UnionFindSetCmp};

const VERTICAL_LENGTH: usize = 2;
const HORIZONTAL_LENGTH: usize = 7;

pub struct Maze {
    rows: usize,
    columns: usize,
    horizontal_walls: Grid<bool>,
    vertical_walls: Grid<bool>,
}

fn write_horizontal_row(
    f: &mut std::fmt::Formatter<'_>,
    maze: &Maze,
    row_idx: usize,
) -> std::fmt::Result {
    write!(f, "| ")?;

    for j in 0..maze.horizontal_walls.cols() - 1 {
        write_single_horizontal_wall(f, maze, row_idx, j)?;

        write!(f, " + ")?;
    }
    write_single_horizontal_wall(f, maze, row_idx, maze.horizontal_walls.cols() - 1)?;

    write!(f, " |\n")
}

fn write_single_horizontal_wall(
    f: &mut std::fmt::Formatter,
    maze: &Maze,
    row_idx: usize,
    j: usize,
) -> Result<(), std::fmt::Error> {
    let is_wall = if maze.horizontal_walls[row_idx][j] {
        "-"
    } else {
        " "
    };
    write_rep(f, is_wall, HORIZONTAL_LENGTH - 2)?;
    Ok(())
}

fn write_rep(f: &mut std::fmt::Formatter<'_>, data: &str, repeat_count: usize) -> std::fmt::Result {
    for _ in 0..repeat_count {
        write!(f, "{data}")?;
    }

    Ok(())
}

fn write_vertical_row(
    f: &mut std::fmt::Formatter<'_>,
    maze: &Maze,
    row_idx: usize,
) -> std::fmt::Result {
    write!(f, "| ")?;

    for j in 0..maze.vertical_walls.cols() {
        if maze.vertical_walls[row_idx][j] {
            write_rep(f, " ", HORIZONTAL_LENGTH - 1)?;
            write!(f, "| ")?;
        } else {
            write_rep(f, " ", HORIZONTAL_LENGTH + 1)?;
        }
    }

    write_rep(f, " ", HORIZONTAL_LENGTH - 2)?;
    write!(f, " |\n")
}

fn write_border_row(
    f: &mut std::fmt::Formatter<'_>,
    first_char: char,
    end_char: char,
    num_columns: usize,
) -> std::fmt::Result {
    write!(f, "{first_char}")?;

    let num_dashes = (HORIZONTAL_LENGTH + 1) * num_columns - 1;
    for _ in 0..num_dashes {
        write!(f, "-")?;
    }

    write!(f, "{end_char}")?;
    write!(f, "\n")
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_border_row(f, '/', '\\', self.columns)?;

        for i in 0..self.rows - 1 {
            for _ in 0..VERTICAL_LENGTH {
                write_vertical_row(f, &self, i)?;
            }
            write_horizontal_row(f, &self, i)?;
        }
        write_vertical_row(f, &self, self.rows - 1)?;
        write_vertical_row(f, &self, self.rows - 1)?;

        write_border_row(f, '\\', '/', self.columns)
    }
}

impl Maze {
    pub fn full(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            horizontal_walls: Grid::<bool>::init(rows - 1, columns, true),
            vertical_walls: Grid::<bool>::init(rows, columns - 1, true),
        }
    }

    pub fn empty(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            horizontal_walls: Grid::<bool>::init(rows - 1, columns, false),
            vertical_walls: Grid::<bool>::init(rows, columns - 1, false),
        }
    }

    pub fn random(rows: usize, columns: usize) -> Self {
        enum WallType {
            Vertical,
            Horizontal,
        }

        struct WallIndex {
            i: usize,
            j: usize,
            wall_type: WallType,
        }

        let mut rng = rand::thread_rng();

        // Generate a random order of maze walls
        let mut wall_indices = Vec::new();
        for i in 0..rows - 1 {
            for j in 0..columns {
                wall_indices.push(WallIndex {
                    i,
                    j,
                    wall_type: WallType::Horizontal,
                });
            }
        }
        for i in 0..rows {
            for j in 0..columns - 1 {
                wall_indices.push(WallIndex {
                    i,
                    j,
                    wall_type: WallType::Vertical,
                });
            }
        }

        wall_indices.shuffle(&mut rng);

        // Add the cells of the maze into a union-find structure
        let mut cells_connected_areas = FastUnionFind::new();
        let mut cells = Grid::new(rows, columns);
        for i in 0..rows {
            for j in 0..columns {
                cells[i][j] = cells_connected_areas.add_element();
            }
        }

        // Start consuming the shuffled walls, "connecting" between the cells one at a time
        let mut vertical_walls = Grid::init(rows, columns - 1, true);
        let mut horizontal_walls = Grid::init(rows - 1, columns, true);

        for wall in wall_indices {
            let (neighbor_i, neighbor_j) = match wall.wall_type {
                WallType::Vertical => (wall.i, wall.j + 1),
                WallType::Horizontal => (wall.i + 1, wall.j),
            };

            let node = cells[wall.i][wall.j];
            let neighbor = cells[neighbor_i][neighbor_j];

            if !cells_connected_areas.is_in_same_set(node, neighbor) {
                cells_connected_areas.union(node, neighbor);
                match wall.wall_type {
                    WallType::Vertical => vertical_walls[wall.i][wall.j] = false,
                    WallType::Horizontal => horizontal_walls[wall.i][wall.j] = false,
                }
            }
        }

        Self {
            rows,
            columns,
            vertical_walls,
            horizontal_walls,
        }
    }
}
