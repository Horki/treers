pub mod bst;
pub mod rbtree;

pub trait SedgewickMap<K: Ord, V> {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn get(&self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V);
    fn height(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains(&self, key: &K) -> bool;
    fn min(&self) -> Option<&K>;
    fn max(&self) -> Option<&K>;
}

#[cfg(test)]
mod tests {
    use super::bst::BST;
    use super::SedgewickMap;
    use crate::rbtree::RedBlackTree;

    #[test]
    fn its_42() {
        assert_eq!(20 + 22, 42);
    }

    fn is_empty<K: Ord, V>(map: &impl SedgewickMap<K, V>) -> bool {
        map.is_empty()
    }

    #[test]
    fn test_empty() {
        let bst: BST<i32, i32> = BST::new();
        let rbt: RedBlackTree<i32, i32> = RedBlackTree::new();
        assert_eq!(is_empty(&bst), true);
        assert_eq!(is_empty(&rbt), true);
    }
}
