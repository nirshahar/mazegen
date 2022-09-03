use crate::union_find::{UnionFind, UnionFindSetCmp, A};

mod union_find;

fn main() {
    let mut sets = A::new();

    let a = sets.add_element();
    let b = sets.add_element();

    println!(
        "before union, is in the same set: {}",
        sets.is_in_same_set(a, b)
    );

    sets.union(a, b);

    println!(
        "after union, is in the same set: {}",
        sets.is_in_same_set(a, b)
    );
}
