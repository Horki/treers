#![feature(test)]
extern crate test;

use std::collections::BTreeMap;
use test::Bencher;
use treers::bst::BST;
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
