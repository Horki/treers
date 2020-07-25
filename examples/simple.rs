extern crate treers;

use treers::bst::BST;
use treers::{SedgewickMap, Traversals, TreeTraversal};

fn main() {
    let mut bst: BST<char, i32> = BST::new();
    bst.put('c', 3);
    bst.put('d', 4);
    bst.put('b', 2);
    bst.put('a', 1);
    println!("min = {}", bst.min().unwrap());
    println!("max = {}", bst.max().unwrap());
    println!("pre order traversal");
    for (a, _) in bst.traverse(&Traversals::PreOrder) {
        print!("{}, ", *a);
    }
    println!("\nin order traversal");
    for (a, _) in bst.traverse(&Traversals::InOrder) {
        print!("{}, ", *a);
    }
    println!("\npost order traversal");
    for (a, _) in bst.traverse(&Traversals::PostOrder) {
        print!("{}, ", *a);
    }
    println!("\nlevel order traversal");
    for (a, _) in bst.traverse(&Traversals::LevelOrder) {
        print!("{}, ", *a);
    }
    println!();
}
