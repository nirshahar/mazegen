use rand::seq::SliceRandom;

use grid::Grid;

use crate::union_find::{FastUnionFind, UnionFind, UnionFindSetCmp};

pub struct Maze {
    horizontal_walls: Grid<bool>,
    vertical_walls: Grid<bool>,
}

impl Maze {
    fn full(rows: usize, columns: usize) -> Self {
        Self {
            horizontal_walls: Grid::<bool>::init(rows - 1, columns, true),
            vertical_walls: Grid::<bool>::init(rows, columns - 1, true),
        }
    }

    fn empty(rows: usize, columns: usize) -> Self {
        Self {
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
            vertical_walls,
            horizontal_walls,
        }
    }
}
