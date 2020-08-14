use treers::btree::BalancedTree;
use treers::SedgewickMap;

fn main() {
    let mut btree = BalancedTree::new();
    btree.put(4, 5);
    btree.put(2, 1);
    btree.put(3, 3);
    btree.put(1, 4);
    println!("min(1) = {}", btree.min().unwrap());
    println!("min(4) = {}", btree.max().unwrap());
    println!("btree[{}] = {}", 1, btree.get(&1).unwrap());
    dbg!(btree);
}
