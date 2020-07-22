#![feature(test)]
extern crate test;

use std::collections::BTreeMap;
use test::Bencher;
use treers::bst::BST;
use treers::rbtree::RedBlackTree;
use treers::SedgewickMap;

#[bench]
fn bst_add_one_million(b: &mut Bencher) {
    let mut bst = BST::new();
    b.iter(|| {
        for i in 1..=1_000_u64 {
            bst.put(i, i + 1);
        }
    });
}

#[bench]
fn std_btree_add_one_million(b: &mut Bencher) {
    let mut btree = BTreeMap::new();
    b.iter(|| {
        for i in 1..=1_000_u64 {
            btree.insert(i, i + 1);
        }
    });
}

#[bench]
fn rbtree_add_one_million_left_rotate(b: &mut Bencher) {
    let mut rbtree: RedBlackTree<u64, u64> = RedBlackTree::new();
    b.iter(|| {
        for i in 1..=1_000_u64 {
            rbtree.put(i, i + 1);
        }
    });
}

#[bench]
#[ignore]
fn rbtree_add_one_million_right_rotate(b: &mut Bencher) {
    // TODO: fix, right rotate
    let mut rbtree = RedBlackTree::new();
    b.iter(|| {
        for i in (1..=1_000_u64).rev() {
            rbtree.put(i, i + 1);
        }
    });
}
