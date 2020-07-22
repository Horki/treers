use treers::rbtree::RedBlackTree;
use treers::SedgewickMap;

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
    println!("height = {}", rbtree.height());
}

// TODO: fix right rotate
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
    println!("size = {}", rbtree.size());
    println!("height = {}", rbtree.height());
}

fn main() {
    println!("Working left rotate");
    left_rotate();

    println!("Non-Working right rotate");
    right_rotate();
}
