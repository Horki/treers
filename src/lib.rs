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

    /// A immutable recursive traversals over Binary Trees.
    ///
    /// `Pre order`
    /// `In order`
    /// `Post order`
    /// `Level order`
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::{SedgewickMap, Traversals};
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// bst.put('c', 3);
    /// bst.put('d', 4);
    /// bst.put('b', 2);
    /// bst.put('a', 1);
    ///
    /// // Pre order Traversal by keys
    /// for (a, _) in bst.traverse(&Traversals::PreOrder) {
    ///     print!("{}, ", *a);
    /// }
    ///
    /// // In order Traversal by keys
    /// for (a, _) in bst.traverse(&Traversals::InOrder) {
    ///     print!("{}, ", *a);
    /// }
    ///
    /// // Post order Traversal by keys
    /// for (a, _) in bst.traverse(&Traversals::PostOrder) {
    ///     print!("{}, ", *a);
    /// }
    ///
    /// // Level order Traversal by keys
    /// for (a, _) in bst.traverse(&Traversals::LevelOrder) {
    ///     print!("{}, ", *a);
    /// }
    /// ```
    fn traverse(&self, traverse: &Traversals) -> std::vec::IntoIter<(&K, &V)> {
        let mut vec = Vec::with_capacity(self.size());
        match traverse {
            Traversals::PreOrder => self.pre_order(&mut vec),
            Traversals::InOrder => self.in_order(&mut vec),
            Traversals::PostOrder => self.post_order(&mut vec),
            Traversals::LevelOrder => {
                for level in 1..=self.height() {
                    self.level_order(&mut vec, level);
                }
            }
        }
        vec.into_iter()
    }
    // TODO: figure out how to make this as a different trait!
    fn pre_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>);
    fn in_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>);
    fn post_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>);
    fn level_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>, level: usize);
}

pub enum Traversals {
    PreOrder,
    InOrder,
    PostOrder,
    LevelOrder,
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
