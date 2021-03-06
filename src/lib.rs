#![forbid(dead_code, unsafe_code, unstable_features)]
#![deny(
    clippy::missing_const_for_fn,
    clippy::redundant_pub_crate,
    future_incompatible,
    private_in_public,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes
)]
#![allow(clippy::use_self)]

pub mod bst;
pub mod btree;
pub mod rbtree;

pub trait SedgewickMap<K: Ord, V> {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn get(&self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V);
    fn height(&self) -> Option<usize>;
    fn is_empty(&self) -> bool {
        self.size().eq(&0_usize)
    }

    /// Checks if key exists in `Tree`
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    /// use treers::rbtree::RedBlackTree;
    /// use treers::btree::BalancedTree;
    ///
    /// let mut bst: BST<char, u32> = BST::new();
    /// let mut rbtree: RedBlackTree<char, u32> = RedBlackTree::new();
    /// let mut btree: BalancedTree<char, u32> = BalancedTree::new();
    ///
    /// bst.put('a', 2);
    /// rbtree.put('a', 2);
    /// btree.put('a', 2);
    ///
    /// assert!(bst.contains(&'a'));
    /// assert_eq!(bst.contains(&'b'), false);
    ///
    /// assert!(rbtree.contains(&'a'));
    /// assert_eq!(rbtree.contains(&'b'), false);
    ///
    /// assert!(btree.contains(&'a'));
    /// assert_eq!(btree.contains(&'b'), false);
    /// ```
    fn contains(&self, key: &K) -> bool {
        self.get(&key).is_some()
    }
    fn min(&self) -> Option<&K>;
    fn max(&self) -> Option<&K>;
}

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
/// use treers::{SedgewickMap, Traversals, TreeTraversal};
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
pub trait TreeTraversal<K: Ord, V>: SedgewickMap<K, V> {
    fn traverse(&self, traverse: &Traversals) -> std::vec::IntoIter<(&K, &V)> {
        let mut vec = Vec::with_capacity(self.size());
        match traverse {
            Traversals::PreOrder => self.pre_order(&mut vec),
            Traversals::InOrder => self.in_order(&mut vec),
            Traversals::PostOrder => self.post_order(&mut vec),
            Traversals::LevelOrder => {
                for level in 0..=self.height().unwrap() {
                    self.level_order(&mut vec, level);
                }
            }
        }
        vec.into_iter()
    }
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
    use crate::bst::BST;
    use crate::btree::BalancedTree;
    use crate::rbtree::RedBlackTree;
    use crate::SedgewickMap;

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
        let btree: BalancedTree<i32, i32> = BalancedTree::new();

        assert!(is_empty(&bst));
        assert!(is_empty(&rbt));
        assert!(is_empty(&btree));
    }
}
