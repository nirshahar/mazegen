use clap::Parser;

use crate::maze::Maze;

mod maze;
mod union_find;

#[derive(Parser, Debug)]
#[clap(author, version="1.0.0", about, long_about = None)]
struct Args {
    // The number of rows in the maze (the height of the maze)
    #[clap(short, long, value_parser)]
    rows: usize,

    // The number of columns in the maze (the width of the maze)
    #[clap(short, long, value_parser)]
    columns: usize,
}

fn main() {
    let args = Args::parse();

    println!("{}", Maze::random(args.rows, args.columns));
}
