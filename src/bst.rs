use crate::{SedgewickMap, TreeTraversal};
use std::cmp::Ordering;
use std::ops::Index;

/// 3.2 Binary Search Tree
///
/// BST implementation from Robert Sedgewick book, "Algorithms" 4th edition
///
/// # Examples
///
/// ```
/// use treers::bst::BST;
/// use treers::SedgewickMap;
///
/// let mut bst: BST<char, i32> = BST::new();
/// bst.put('c', 3);
/// bst.put('d', 4);
/// bst.put('b', 2);
/// bst.put('a', 1);
///
/// // Generate a unbalanced Binary Search Tree
/// //    c
/// //   / \
/// //  b   d
/// // /
/// // a
///
/// // Gets a value 1
/// println!("bst[a] = {}", bst.get(&'a').unwrap());
/// assert_eq!(bst.height(), Some(2_usize));
/// ```
#[derive(Debug)]
pub enum BST<K: Ord, V> {
    Node {
        k: K,
        v: V,
        size: usize,
        left: Box<BST<K, V>>,
        right: Box<BST<K, V>>,
    },
    NIL,
}

impl<K: Ord, V> SedgewickMap<K, V> for BST<K, V> {
    /// Inits a new instance of Binary Search Tree.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    ///
    /// let bst: BST<char, i32> = BST::new();
    /// assert!(bst.is_empty());
    /// ```
    fn new() -> Self {
        BST::NIL
    }

    /// Returns a size of elements in `BST`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// assert_eq!(bst.size(), 0_usize);
    /// bst.put('a', 1);
    /// bst.put('b', 2);
    /// bst.put('c', 3);
    /// bst.put('d', 4);
    /// assert_eq!(bst.size(), 4_usize);
    /// ```
    fn size(&self) -> usize {
        match &self {
            BST::Node {
                k: _,
                v: _,
                ref size,
                left: _,
                right: _,
            } => *size,
            _ => 0_usize,
        }
    }

    /// Returns a reference to optional reference to value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// bst.put('a', 1);
    /// assert_eq!(bst.get(&'a'), Some(&1));
    /// assert_eq!(bst.get(&'b'), None);
    /// assert_eq!(bst[&'a'], 1);
    /// ```
    fn get(&self, key: &K) -> Option<&V> {
        match self {
            BST::Node {
                ref k,
                ref v,
                size: _,
                ref left,
                ref right,
            } => match key.cmp(k) {
                Ordering::Less => left.get(key),
                Ordering::Greater => right.get(key),
                _ => Some(v),
            },
            _ => None,
        }
    }

    /// Insert a key-value pair into the `BST`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// assert!(bst.is_empty());
    ///
    /// bst.put('a', 1_i32);
    /// assert_eq!(bst.is_empty(), false);
    /// assert_eq!(bst.get(&'a'), Some(&1_i32));
    /// assert_eq!(bst[&'a'], 1_i32);
    /// ```
    fn put(&mut self, key: K, value: V) {
        match self {
            BST::Node {
                ref k,
                v: _,
                ref mut size,
                ref mut left,
                ref mut right,
            } => {
                match key.cmp(k) {
                    Ordering::Less => left.put(key, value),
                    Ordering::Greater => right.put(key, value),
                    _ => {}
                }
                *size = 1_usize + left.size() + right.size();
            }
            BST::NIL => {
                // Insert a leaf node
                *self = BST::Node {
                    k: key,
                    v: value,
                    size: 1,
                    left: Box::new(BST::NIL),
                    right: Box::new(BST::NIL),
                }
            }
        }
    }

    /// Get height of `BST`.
    ///
    /// BST is not balanced tree, so in worst-case scenario, height will be
    /// same as size, like a Linked-List.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// bst.put('a', 1);
    /// bst.put('b', 2);
    /// bst.put('c', 3);
    /// bst.put('d', 4);
    /// //  a         <-- height: 0
    /// // / \
    /// //    b       <-- height: 1
    /// //   / \
    /// //      c     <-- height: 2
    /// //     / \
    /// //        d   <-- height: 3
    /// // Note -The Height of binary tree with single node is taken as zero.
    /// assert_eq!(bst.get(&'a'), Some(&1_i32));
    /// assert_eq!(bst.height(), Some(3_usize));
    /// assert_eq!(bst.size(), 4_usize);
    /// ```
    fn height(&self) -> Option<usize> {
        let h = self.get_height();
        if h > 0_usize {
            Some(h - 1)
        } else {
            None
        }
    }

    /// Checks if `BST` node is empty.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// assert!(bst.is_empty());
    /// bst.put('a', 2);
    /// assert_eq!(bst.is_empty(), false);
    /// ```
    fn is_empty(&self) -> bool {
        !matches!(*self, BST::Node { .. })
    }

    /// Returns a optional reference to minimal key
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// assert_eq!(bst.min(), None);
    /// bst.put('c', 1);
    /// bst.put('a', 2);
    /// bst.put('b', 3);
    /// bst.put('d', 4);
    /// assert_eq!(bst.min(), Some(&'a'));
    /// ```
    fn min(&self) -> Option<&K> {
        match self {
            BST::Node {
                ref k,
                v: _,
                size: _,
                ref left,
                right: _,
            } => {
                if let Some(l) = left.min() {
                    Some(l)
                } else {
                    Some(k)
                }
            }
            _ => None,
        }
    }

    /// Returns a optional reference to maximum key
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::SedgewickMap;
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// assert_eq!(bst.max(), None);
    /// bst.put('c', 1);
    /// bst.put('a', 2);
    /// bst.put('b', 3);
    /// bst.put('d', 4);
    /// assert_eq!(bst.max(), Some(&'d'));
    /// ```
    fn max(&self) -> Option<&K> {
        match self {
            BST::Node {
                ref k,
                v: _,
                size: _,
                left: _,
                ref right,
            } => {
                if let Some(l) = right.max() {
                    Some(l)
                } else {
                    Some(k)
                }
            }
            _ => None,
        }
    }
}

impl<K: Ord + Clone, V: Clone> TreeTraversal<K, V> for BST<K, V> {
    /// Returns traverse pre ordered
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::{SedgewickMap, TreeTraversal, Traversals};
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// bst.put('c', 3);
    /// bst.put('d', 4);
    /// bst.put('b', 2);
    /// bst.put('a', 1);
    /// //    c
    /// //   / \
    /// //  b   d
    /// // /
    /// // a
    /// assert_eq!(bst.traverse(&Traversals::PreOrder).as_slice(),
    ///       &[(&'c', &3), (&'b', &2), (&'a', &1), (&'d', &4)]);
    /// ```
    fn pre_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
        if let BST::Node {
            ref k,
            ref v,
            size: _,
            ref left,
            ref right,
        } = self
        {
            vec.push((k, v));
            left.pre_order(vec);
            right.pre_order(vec);
        }
    }

    /// Returns traverse in ordered
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::{SedgewickMap, TreeTraversal, Traversals};
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// bst.put('c', 3);
    /// bst.put('d', 4);
    /// bst.put('b', 2);
    /// bst.put('a', 1);
    /// //    c
    /// //   / \
    /// //  b   d
    /// // /
    /// // a
    /// assert_eq!(bst.traverse(&Traversals::InOrder).as_slice(),
    ///       &[(&'a', &1), (&'b', &2), (&'c', &3), (&'d', &4)]);
    /// ```
    fn in_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
        if let BST::Node {
            ref k,
            ref v,
            size: _,
            ref left,
            ref right,
        } = self
        {
            left.in_order(vec);
            vec.push((k, v));
            right.in_order(vec);
        }
    }

    /// Returns traverse post ordered
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::{SedgewickMap, TreeTraversal, Traversals};
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// bst.put('c', 3);
    /// bst.put('d', 4);
    /// bst.put('b', 2);
    /// bst.put('a', 1);
    /// //    c
    /// //   / \
    /// //  b   d
    /// // /
    /// // a
    /// assert_eq!(bst.traverse(&Traversals::PostOrder).as_slice(),
    ///       &[(&'a', &1), (&'b', &2), (&'d', &4), (&'c', &3)]);
    /// ```
    fn post_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
        if let BST::Node {
            ref k,
            ref v,
            size: _,
            ref left,
            ref right,
        } = self
        {
            left.post_order(vec);
            right.post_order(vec);
            vec.push((k, v));
        }
    }

    /// Returns traverse level ordered
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::bst::BST;
    /// use treers::{SedgewickMap, TreeTraversal, Traversals};
    ///
    /// let mut bst: BST<char, i32> = BST::new();
    /// bst.put('c', 3);
    /// bst.put('d', 4);
    /// bst.put('b', 2);
    /// bst.put('a', 1);
    /// //    c
    /// //   / \
    /// //  b   d
    /// // /
    /// // a
    /// assert_eq!(bst.traverse(&Traversals::LevelOrder).as_slice(),
    ///       &[(&'c', &3), (&'b', &2), (&'d', &4), (&'a', &1)]);
    /// ```
    fn level_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>, level: usize) {
        if let BST::Node {
            ref k,
            ref v,
            size: _,
            ref left,
            ref right,
        } = self
        {
            match level {
                0 => vec.push((k, v)),
                _ => {
                    left.level_order(vec, level - 1);
                    right.level_order(vec, level - 1);
                }
            }
        }
    }
}

// internal methods
impl<K: Ord, V> BST<K, V> {
    fn get_height(&self) -> usize {
        match self {
            BST::Node {
                k: _,
                v: _,
                size: _,
                ref left,
                ref right,
            } => 1_usize + std::cmp::max(left.get_height(), right.get_height()),
            _ => 0_usize,
        }
    }
    /// Easter egg: invert a BST :)
    pub fn invert(&mut self) {
        if let BST::Node {
            k: _,
            v: _,
            size: _,
            ref mut left,
            ref mut right,
        } = self
        {
            left.invert();
            right.invert();
            std::mem::swap(left, right);
        }
    }
}

impl<K: Ord + Clone, V: Clone> Default for BST<K, V> {
    /// Creates an empty `BST<K, V>`.
    fn default() -> BST<K, V> {
        BST::new()
    }
}

impl<K: Ord + Clone, V: Clone> Index<&K> for BST<K, V> {
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `BST`.
    #[inline]
    fn index(&self, index: &K) -> &V {
        self.get(index).expect("Missing entry for key in BST")
    }
}

#[cfg(test)]
mod tests {
    use super::{SedgewickMap, BST};
    use crate::{Traversals, TreeTraversal};

    #[test]
    fn test_is_empty() {
        let bst: BST<i32, i32> = BST::new();
        assert!(bst.is_empty());
    }

    #[test]
    fn test_is_not_empty() {
        let mut bst: BST<i32, i32> = BST::new();
        bst.put(1, 2);
        bst.put(2, 3);
        assert_eq!(bst.is_empty(), false);
    }

    #[test]
    fn test_size_zero() {
        let bst: BST<i32, i32> = BST::new();
        assert_eq!(bst.size(), 0);
        assert_eq!(bst.height(), None);
    }

    #[test]
    fn test_put() {
        let mut bst: BST<u32, Vec<i32>> = BST::new();
        let v = vec![1, 2, 3];
        bst.put(1, v);
        assert_eq!(bst.get(&1_u32), Some(&vec![1_i32, 2, 3]));
    }

    #[test]
    fn test_get() {
        let mut bst: BST<u32, i32> = BST::new();
        bst.put(1_u32, -1_i32);
        assert_eq!(bst.get(&1_u32), Some(&-1_i32));
        assert_eq!(bst.get(&10_u32), None);
        assert_eq!(bst[&1_u32], -1_i32);
    }

    #[test]
    #[should_panic(expected = "Missing entry for key in BST")]
    fn test_index_panic() {
        let mut bst: BST<i32, i32> = BST::new();
        bst.put(1_i32, -1_i32);
        let _panics = bst[&10_i32];
    }

    #[test]
    fn test_contains() {
        let mut bst: BST<i32, i32> = BST::new();
        bst.put(1_i32, -1_i32);
        assert!(bst.contains(&1_i32));
        assert_eq!(bst.contains(&-1_i32), false);
    }

    #[test]
    fn test_size_two() {
        let mut bst: BST<i32, i32> = BST::new();
        bst.put(1, 2);
        bst.put(2, 3);
        assert_eq!(bst.size(), 2);
    }

    #[test]
    fn test_size_and_height_one_thousand() {
        let mut bst: BST<u64, u64> = BST::new();
        for i in 1..=1_000_u64 {
            bst.put(i, i + 1);
        }
        assert_eq!(bst.size(), 1_000);
        assert_eq!(bst.height(), Some(999));
    }

    #[test]
    fn test_min() {
        let mut bst: BST<u32, u32> = BST::new();
        assert_eq!(bst.min(), None);
        for i in vec![6_u32, 4, 5, 2, 1, 3] {
            bst.put(i, i);
        }
        assert_eq!(bst.min(), Some(&1_u32));
        assert_eq!(bst.get(bst.min().unwrap()), bst.min());
    }

    #[test]
    fn test_max() {
        let mut bst: BST<u32, u32> = BST::new();
        assert_eq!(bst.max(), None);
        for i in vec![6_u32, 4, 5, 2, 1, 3] {
            bst.put(i, i);
        }
        assert_eq!(bst.max(), Some(&6_u32));
        assert_eq!(bst.get(bst.max().unwrap()), bst.max());
    }

    #[test]
    fn test_in_order() {
        let mut bst: BST<char, i32> = BST::new();
        let res = vec!['a', 'b', 'c', 'd'];
        let mut it = res.iter();
        bst.put('c', 3);
        bst.put('d', 4);
        bst.put('b', 2);
        bst.put('a', 1);
        assert_eq!(bst.size(), res.len());
        for (a, _) in bst.traverse(&Traversals::InOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_pre_order() {
        let mut bst: BST<char, i32> = BST::new();
        let res = vec!['c', 'b', 'a', 'd'];
        let mut it = res.iter();
        bst.put('c', 3);
        bst.put('d', 4);
        bst.put('b', 2);
        bst.put('a', 1);
        assert_eq!(bst.size(), res.len());
        for (a, _) in bst.traverse(&Traversals::PreOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_post_order() {
        let mut bst: BST<char, i32> = BST::new();
        let res = vec!['a', 'b', 'd', 'c'];
        let mut it = res.iter();
        bst.put('c', 3);
        bst.put('d', 4);
        bst.put('b', 2);
        bst.put('a', 1);
        assert_eq!(bst.size(), res.len());
        for (a, _) in bst.traverse(&Traversals::PostOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_level_order() {
        //     c
        //    / \
        //   b   d
        //  /
        // a
        //
        // Inverted BST
        //     c
        //    / \
        //   d   b
        //    \
        //     a
        let mut bst: BST<char, i32> = BST::new();
        let res = vec!['c', 'b', 'd', 'a'];
        let mut it = res.iter();
        bst.put('c', 3);
        bst.put('d', 4);
        bst.put('b', 2);
        bst.put('a', 1);
        assert_eq!(bst.size(), res.len());
        for (a, _) in bst.traverse(&Traversals::LevelOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
        bst.invert();
        let res = vec!['c', 'd', 'b', 'a'];
        it = res.iter();
        assert_eq!(bst.size(), res.len());
        for (a, _) in bst.traverse(&Traversals::LevelOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }

    }
}
