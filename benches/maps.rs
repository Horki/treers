#[macro_use]
extern crate bencher;

use bencher::Bencher;
use std::collections::BTreeMap;
use treers::bst::BST;
use treers::btree::BalancedTree;
use treers::rbtree::RedBlackTree;
use treers::SedgewickMap;

fn bst_add_one_thousand_left_rotate(b: &mut Bencher) {
    let mut bst = BST::new();
    b.iter(|| {
        for i in 1..=1_000_u64 {
            bst.put(i, i + 1);
        }
    });
}

fn bst_add_one_thousand_right_rotate(b: &mut Bencher) {
    let mut bst = BST::new();
    b.iter(|| {
        for i in (1..=1_000_u64).rev() {
            bst.put(i, i + 1);
        }
    });
}

fn std_btree_add_one_thousand_left_rotate(b: &mut Bencher) {
    let mut btree = BTreeMap::new();
    b.iter(|| {
        for i in 1..=1_000_u64 {
            btree.insert(i, i + 1);
        }
    });
}

fn std_btree_add_one_thousand_right_rotate(b: &mut Bencher) {
    let mut btree = BTreeMap::new();
    b.iter(|| {
        for i in (1..=1_000_u64).rev() {
            btree.insert(i, i + 1);
        }
    });
}

fn rbtree_add_one_thousand_left_rotate(b: &mut Bencher) {
    let mut rbtree = RedBlackTree::new();
    b.iter(|| {
        for i in 1..=1_000_u64 {
            rbtree.put(i, i + 1);
        }
    });
}

fn rbtree_add_one_thousand_right_rotate(b: &mut Bencher) {
    let mut rbtree = RedBlackTree::new();
    b.iter(|| {
        for i in (1..=1_000_u64).rev() {
            rbtree.put(i, i + 1);
        }
    });
}

fn btree_add_one_thousand_left_rotate(b: &mut Bencher) {
    let mut btree = BalancedTree::new();
    b.iter(|| {
        for i in 1..=1000_u64 {
            btree.put(i, i + 1);
        }
    });
}

fn btree_add_one_thousand_right_rotate(b: &mut Bencher) {
    let mut btree = BalancedTree::new();
    b.iter(|| {
        for i in (1..=1000_u64).rev() {
            btree.put(i, i + 1);
        }
    });
}

benchmark_group!(
    benches,
    bst_add_one_thousand_left_rotate,
    bst_add_one_thousand_right_rotate,
    std_btree_add_one_thousand_left_rotate,
    std_btree_add_one_thousand_right_rotate,
    rbtree_add_one_thousand_left_rotate,
    rbtree_add_one_thousand_right_rotate,
    btree_add_one_thousand_left_rotate,
    btree_add_one_thousand_right_rotate,
);

benchmark_main!(benches);

// running 8 tests
// test bst_add_one_thousand_left_rotate        ... bench:   3,440,874 ns/iter (+/- 486,622)
// test bst_add_one_thousand_right_rotate       ... bench:   3,476,793 ns/iter (+/- 491,932)
// test btree_add_one_thousand_left_rotate      ... bench:   1,293,973 ns/iter (+/- 289,641)
// test btree_add_one_thousand_right_rotate     ... bench:   1,072,404 ns/iter (+/- 125,027)
// test rbtree_add_one_thousand_left_rotate     ... bench:      92,525 ns/iter (+/- 14,757)
// test rbtree_add_one_thousand_right_rotate    ... bench:      94,400 ns/iter (+/- 63,814)
// test std_btree_add_one_thousand_left_rotate  ... bench:      32,799 ns/iter (+/- 12,486)
// test std_btree_add_one_thousand_right_rotate ... bench:      32,020 ns/iter (+/- 3,540)
