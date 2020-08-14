use treers::rbtree::RedBlackTree;
use treers::{SedgewickMap, Traversals, TreeTraversal};

fn left_rotate() {
    let mut rbtree: RedBlackTree<u32, u32> = RedBlackTree::new();
    rbtree.put(1, 2);
    println!("insert 1");
    dbg!(rbtree.clone());
    rbtree.put(2, 4);
    println!("insert 2, and rotate left");
    dbg!(rbtree.clone());
    rbtree.put(3, 6);
    println!("insert 3");
    dbg!(rbtree.clone());
    rbtree.put(4, 8);
    println!("insert 4");
    dbg!(rbtree.clone());
    rbtree.put(5, 10);
    rbtree.put(6, 12);
    rbtree.put(7, 14);
    rbtree.put(8, 16);
    rbtree.put(9, 18);
    dbg!(rbtree.clone());
    println!("size = {}", rbtree.size());
    println!("height = {}", rbtree.height().unwrap());
    println!("\nIn order");
    for (k, _v) in rbtree.traverse(&Traversals::InOrder) {
        print!("{}, ", k);
    }
    println!("\nPre order");
    for (k, _v) in rbtree.traverse(&Traversals::PreOrder) {
        print!("{}, ", k);
    }
    println!("\nPost order");
    for (k, _v) in rbtree.traverse(&Traversals::PostOrder) {
        print!("{}, ", k);
    }
    println!("\nLevel order");
    for (k, _v) in rbtree.traverse(&Traversals::LevelOrder) {
        print!("{}, ", k);
    }
    println!();
    {
        let mut rbtree_left_max = RedBlackTree::new();
        for i in 1..=1000_u64 {
            rbtree_left_max.put(i, i);
        }
        println!("min = {}", rbtree_left_max.min().unwrap());
        println!("max = {}", rbtree_left_max.max().unwrap());
        println!("500 == {}", rbtree_left_max.get(&500).unwrap());
    }
}

fn right_rotate() {
    let mut rbtree: RedBlackTree<u32, u32> = RedBlackTree::new();
    rbtree.put(9, 2);
    rbtree.put(8, 4);
    rbtree.put(7, 6);
    rbtree.put(6, 8);
    rbtree.put(5, 10);
    rbtree.put(4, 12);
    rbtree.put(3, 14);
    rbtree.put(2, 16);
    rbtree.put(1, 18);
    dbg!(rbtree.clone());
    println!("\nIn order");
    for (k, _v) in rbtree.traverse(&Traversals::InOrder) {
        print!("{}, ", k);
    }
    println!("\nPre order");
    for (k, _v) in rbtree.traverse(&Traversals::PreOrder) {
        print!("{}, ", k);
    }
    println!("\nPost order");
    for (k, _v) in rbtree.traverse(&Traversals::PostOrder) {
        print!("{}, ", k);
    }
    println!("\nLevel order");
    for (k, _v) in rbtree.traverse(&Traversals::LevelOrder) {
        print!("{}, ", k);
    }

    println!("\nsize = {}", rbtree.size());
    println!("height = {}", rbtree.height().unwrap());
    {
        let mut rbtree_right_max = RedBlackTree::new();
        for i in (1..=1000_u64).rev() {
            rbtree_right_max.put(i, i);
        }
        println!("min = {}", rbtree_right_max.min().unwrap());
        println!("max = {}", rbtree_right_max.max().unwrap());
        println!("500 == {}", rbtree_right_max.get(&500).unwrap());
    }
}

fn main() {
    println!("Working left rotate");
    left_rotate();

    println!("Working right rotate");
    right_rotate();
}
