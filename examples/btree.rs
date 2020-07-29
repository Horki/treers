use treers::btree::BalancedTree;
use treers::SedgewickMap;

fn main() {
    let mut btree = BalancedTree::new();
    btree.put(1, 4);
    btree.put(2, 1);
    btree.put(3, 3);
    btree.put(4, 5);
    dbg!(btree);
}