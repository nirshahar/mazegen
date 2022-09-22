use crate::maze::Maze;

mod maze;
mod union_find;

fn main() {
    println!("{}", Maze::random(5, 5));
}
