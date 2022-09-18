use crate::maze::Maze;

mod maze;
mod union_find;

fn main() {
    Maze::random(3, 3);
}
