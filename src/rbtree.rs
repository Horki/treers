use crate::{SedgewickMap, Traversals};
use std::cmp::Ordering;

#[derive(Debug)]
pub enum RedBlackTree<K: Ord + Clone, V: Clone> {
    Node {
        k: K,
        v: V,
        color: bool,
        size: usize,
        left: Box<RedBlackTree<K, V>>,
        right: Box<RedBlackTree<K, V>>,
    },
    NIL,
}

impl<K: Clone + Ord, V: Clone> Clone for RedBlackTree<K, V> {
    fn clone(&self) -> RedBlackTree<K, V> {
        match self {
            RedBlackTree::Node {
                ref k,
                ref v,
                ref color,
                ref size,
                ref left,
                ref right,
            } => RedBlackTree::Node {
                k: k.clone(),
                v: v.clone(),
                color: *color,
                size: *size,
                left: left.clone(),
                right: right.clone(),
            },
            RedBlackTree::NIL => RedBlackTree::NIL,
        }
    }
}

impl<K: Ord + Clone, V: Clone> SedgewickMap<K, V> for RedBlackTree<K, V> {
    fn new() -> Self {
        RedBlackTree::NIL
    }
    fn size(&self) -> usize {
        match &self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                ref size,
                left: _,
                right: _,
            } => *size,
            _ => 0_usize,
        }
    }
    fn get(&self, key: &K) -> Option<&V> {
        match self {
            RedBlackTree::Node {
                ref k,
                ref v,
                color: _,
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

    fn put(&mut self, key: K, value: V) {
        // move values!
        self.insert(&key, &value);
        // set root node to black
        self.set_color(false);
    }

    fn height(&self) -> usize {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                ref left,
                ref right,
            } => 1_usize + std::cmp::max(left.height(), right.height()),
            _ => 0_usize,
        }
    }
    fn is_empty(&self) -> bool {
        match self {
            RedBlackTree::Node { .. } => false,
            _ => true,
        }
    }
    fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    fn min(&self) -> Option<&K> {
        match self {
            RedBlackTree::Node {
                ref k,
                v: _,
                color: _,
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

    fn max(&self) -> Option<&K> {
        match self {
            RedBlackTree::Node {
                ref k,
                v: _,
                color: _,
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

    fn in_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
        if let RedBlackTree::Node {
            ref k,
            ref v,
            color: _,
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
    fn pre_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
        if let RedBlackTree::Node {
            ref k,
            ref v,
            color: _,
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

    fn post_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
        if let RedBlackTree::Node {
            ref k,
            ref v,
            color: _,
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

    fn level_order<'a>(&'a self, vec: &mut Vec<(&'a K, &'a V)>, level: usize) {
        if let RedBlackTree::Node {
            ref k,
            ref v,
            color: _,
            size: _,
            ref left,
            ref right,
        } = self
        {
            match level {
                1 => vec.push((k, v)),
                _ => {
                    left.level_order(vec, level - 1);
                    right.level_order(vec, level - 1);
                }
            }
        }
    }

    // fn traverse(&self, traverse: &Traversals) -> std::vec::IntoIter<(&K, &V)> {
    //     let mut vec = Vec::with_capacity(self.size());
    //     match traverse {
    //         Traversals::PreOrder => self.pre_order(&mut vec),
    //         Traversals::InOrder => self.in_order(&mut vec),
    //         Traversals::PostOrder => self.post_order(&mut vec),
    //         Traversals::LevelOrder => {
    //             for level in 1..=self.height() {
    //                 self.level_order(&mut vec, level);
    //             }
    //         }
    //     }
    //     vec.into_iter()
    // }
}

// TODO: almost same as in BST, refactor later!
/// A immutable recursive traversals over `Red Black Tree`.
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
/// use treers::{SedgewickMap, Traversals};
/// use treers::rbtree::RedBlackTree;
///
/// let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
/// rbtree.put('c', 3);
/// rbtree.put('d', 4);
/// rbtree.put('b', 2);
/// rbtree.put('a', 1);
///
/// // Pre order Traversal by keys
/// for (a, _) in rbtree.traverse(&Traversals::PreOrder) {
///     print!("{}, ", *a);
/// }
///
/// // In order Traversal by keys
/// for (a, _) in rbtree.traverse(&Traversals::InOrder) {
///     print!("{}, ", *a);
/// }
///
/// // Post order Traversal by keys
/// for (a, _) in rbtree.traverse(&Traversals::PostOrder) {
///     print!("{}, ", *a);
/// }
///
/// // Level order Traversal by keys
/// for (a, _) in rbtree.traverse(&Traversals::LevelOrder) {
///     print!("{}, ", *a);
/// }
/// ```
// impl<'a, K: 'a + Ord + Clone, V: 'a + Clone> RedBlackTree<K, V> {
// }

impl<'a, K: 'a + Ord + Clone, V: 'a + Clone> RedBlackTree<K, V> {
    fn insert(&mut self, key: &'a K, value: &'a V) {
        match self {
            RedBlackTree::Node {
                ref mut k,
                ref mut v,
                ref mut color,
                ref mut size,
                ref mut left,
                ref mut right,
            } => {
                match key.cmp(k) {
                    // pass by reference, with same lifetime
                    Ordering::Less => left.insert(&key, &value),
                    Ordering::Greater => right.insert(&key, &value),
                    _ => {}
                }
                // Rotate Left
                if right.is_red() && !left.is_red() {
                    let right_clone = right.clone();
                    *right = right_clone.get_right_clone();
                    *color = true;
                    right.set_color(true);
                    left.set_vals(
                        &k,
                        &v,
                        true,
                        *size,
                        *left.clone(),
                        *right_clone.get_left_clone(),
                    );
                    // Don't move, but use clone, instead
                    *k = key.clone();
                    *v = value.clone();
                }
                // Balance 4-node
                // TODO: fix rotate right later
                if left.is_red() && left.is_left_red() {
                    // println!("rotate right {}", key);
                    // let left_clone = left.clone();
                    // *left = left_clone.get_left_clone();
                    // *color = true;
                    // left.set_color(true);
                    // right.set_vals(
                    //     *k,
                    //     *v,
                    //     true,
                    //     *size,
                    //     *left_clone.get_right_clone(),
                    //     *right.clone(),
                    // );
                    // *k = key;
                    // *v = value;
                }
                // Split 4-node
                // Flip colors
                if left.is_red() && right.is_red() {
                    *color = true;
                    left.set_color(false);
                    right.set_color(false);
                }
                *size = left.size() + right.size() + 1;
            }
            RedBlackTree::NIL => {
                // Insert a leaf node
                *self = RedBlackTree::Node {
                    k: key.clone(),
                    v: value.clone(),
                    color: true,
                    size: 1,
                    left: Box::new(RedBlackTree::NIL),
                    right: Box::new(RedBlackTree::NIL),
                }
            }
        }
    }

    fn set_vals(
        &mut self,
        key: &'a K,
        val: &'a V,
        c: bool,
        s: usize,
        l: RedBlackTree<K, V>,
        r: RedBlackTree<K, V>,
    ) {
        match self {
            RedBlackTree::Node {
                ref mut k,
                ref mut v,
                ref mut color,
                ref mut size,
                ref mut left,
                ref mut right,
            } => {
                *k = key.clone();
                *v = val.clone();
                *color = c;
                *size = s;
                *left = Box::new(l);
                *right = Box::new(r);
            }
            RedBlackTree::NIL => {
                *self = RedBlackTree::Node {
                    k: key.clone(),
                    v: val.clone(),
                    color: c,
                    size: 1,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
        }
    }

    fn set_color(&mut self, c: bool) {
        if let RedBlackTree::Node {
            k: _,
            v: _,
            ref mut color,
            size: _,
            left: _,
            right: _,
        } = self
        {
            *color = c;
        }
    }

    fn is_red(&self) -> bool {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                ref color,
                size: _,
                left: _,
                right: _,
            } => *color,
            _ => false,
        }
    }

    fn is_left_red(&self) -> bool {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                ref left,
                right: _,
            } => left.is_red(),
            _ => false,
        }
    }

    fn get_left_clone(&self) -> Box<RedBlackTree<K, V>> {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                ref left,
                right: _,
            } => left.clone(),
            _ => Box::new(RedBlackTree::NIL),
        }
    }

    fn get_right_clone(&self) -> Box<RedBlackTree<K, V>> {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                left: _,
                ref right,
            } => right.clone(),
            _ => Box::new(RedBlackTree::NIL),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rbtree::RedBlackTree;
    use crate::{SedgewickMap, Traversals};

    #[test]
    fn test_is_empty() {
        let r: RedBlackTree<i32, i32> = RedBlackTree::new();
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    fn test_is_not_empty() {
        let mut rbtree: RedBlackTree<i32, i32> = RedBlackTree::new();
        rbtree.put(1, 2);
        rbtree.put(2, 3);
        assert_eq!(rbtree.is_empty(), false);
    }

    #[test]
    fn test_size_zero() {
        let rbtree: RedBlackTree<i32, i32> = RedBlackTree::new();
        assert_eq!(rbtree.size(), 0);
        assert_eq!(rbtree.height(), 0);
    }

    #[test]
    fn test_put() {
        let mut rbtree: RedBlackTree<u32, Vec<i32>> = RedBlackTree::new();
        let v = vec![1, 2, 3];
        rbtree.put(1, v);
        assert_eq!(rbtree.get(&1_u32), Some(&vec![1_i32, 2, 3]));
    }

    #[test]
    fn test_get() {
        let mut rbtree: RedBlackTree<u32, i32> = RedBlackTree::new();
        rbtree.put(1_u32, -1_i32);
        assert_eq!(rbtree.get(&1_u32), Some(&-1_i32));
    }

    #[test]
    fn test_contains() {
        let mut rbtree: RedBlackTree<i32, i32> = RedBlackTree::new();
        rbtree.put(1_i32, -1_i32);
        assert_eq!(rbtree.contains(&1_i32), true);
        assert_eq!(rbtree.contains(&-1_i32), false);
    }

    #[test]
    fn test_left_rotate_min() {
        let mut rbtree: RedBlackTree<u32, u32> = RedBlackTree::new();
        for i in vec![1_u32, 2, 3, 4, 5, 6] {
            rbtree.put(i, i);
        }
        assert_eq!(rbtree.min(), Some(&1_u32));
        assert_eq!(rbtree.get(rbtree.min().unwrap()), rbtree.min());
    }

    #[test]
    fn test_right_rotate_min() {
        let mut rbtree: RedBlackTree<u32, u32> = RedBlackTree::new();
        for i in (1..=6).rev() {
            rbtree.put(i, i);
        }
        assert_eq!(rbtree.min(), Some(&1_u32));
        assert_eq!(rbtree.get(rbtree.min().unwrap()), rbtree.min());
    }

    #[test]
    fn test_shuffle_max() {
        let mut rbtree: RedBlackTree<u32, u32> = RedBlackTree::new();
        for i in vec![6_u32, 4, 5, 2, 1, 3] {
            rbtree.put(i, i);
        }
        assert_eq!(rbtree.max(), Some(&6_u32));
        assert_eq!(rbtree.get(rbtree.max().unwrap()), rbtree.max());
    }

    #[test]
    fn test_right_rotate_max() {
        let mut rbtree: RedBlackTree<u32, u32> = RedBlackTree::new();
        for i in (1..=6).rev() {
            rbtree.put(i, i);
        }
        assert_eq!(rbtree.max(), Some(&6_u32));
        assert_eq!(rbtree.get(rbtree.max().unwrap()), rbtree.max());
    }

    #[test]
    fn test_random_in_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['a', 'b', 'c', 'd'];
        let mut it = res.iter();
        rbtree.put('c', 3);
        rbtree.put('d', 4);
        rbtree.put('b', 2);
        rbtree.put('a', 1);
        for (a, _) in rbtree.traverse(&Traversals::InOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    #[ignore]
    fn test_random_pre_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['c', 'b', 'a', 'd'];
        let mut it = res.iter();
        rbtree.put('c', 3);
        rbtree.put('d', 4);
        rbtree.put('b', 2);
        rbtree.put('a', 1);
        for (a, _) in rbtree.traverse(&Traversals::PreOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    #[ignore]
    fn test_random_post_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['a', 'b', 'd', 'c'];
        let mut it = res.iter();
        rbtree.put('c', 3);
        rbtree.put('d', 4);
        rbtree.put('b', 2);
        rbtree.put('a', 1);
        for (a, _) in rbtree.traverse(&Traversals::PostOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    #[ignore]
    fn test_random_level_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['c', 'b', 'd', 'a'];
        let mut it = res.iter();
        rbtree.put('c', 3);
        rbtree.put('d', 4);
        rbtree.put('b', 2);
        rbtree.put('a', 1);
        for (a, _) in rbtree.traverse(&Traversals::LevelOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

}
