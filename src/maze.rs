use grid::Grid;

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
}
