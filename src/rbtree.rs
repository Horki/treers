use crate::{SedgewickMap, TreeTraversal};
use std::cmp::Ordering;

/// 3.3 Balanced Search Trees: Red-Black BST
///
/// Red-Black BST implementation from Robert Sedgewick book, "Algorithms" 4th edition
///
/// # Examples
///
/// ```
/// use treers::SedgewickMap;
/// use treers::rbtree::RedBlackTree;
///
/// let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
/// rbtree.put('a', 1);
/// rbtree.put('b', 2);
/// rbtree.put('c', 3);
/// rbtree.put('d', 4);
/// rbtree.put('e', 5);
/// rbtree.put('f', 6);
///
/// // Generate a balanced Red-Black Binary Search Tree
/// //          d(B)           <-- height: 0
/// //        /      \
/// //     (R)b       f(B)     <-- height: 1
/// //      / \      /    \
/// //   (B)a  c(B) e(R)       <-- height: 2
/// // Note -The Height of binary tree with single node is taken as zero.
/// assert_eq!(rbtree.get(&'a'), Some(&1_i32));
/// assert_eq!(rbtree.height(), Some(2_usize));
/// assert_eq!(rbtree.size(), 6_usize);
/// ```
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

    fn height(&self) -> Option<usize> {
        let height_rbtree = self.get_height();
        if height_rbtree > 0_usize {
            Some(height_rbtree - 1)
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            RedBlackTree::Node { .. } => false,
            _ => true,
        }
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
}

impl<K: Ord + Clone, V: Clone> TreeTraversal<K, V> for RedBlackTree<K, V> {
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
                    let right_size = right_clone.size();
                    *color = right_clone.is_right_red();
                    left.set_vals(
                        &k,
                        &v,
                        true,
                        right_size,
                        *left.clone(),
                        *right_clone.get_left_clone(),
                    );
                    // Don't move, but use clone, instead
                    if let Some(kk) = right_clone.get_key() {
                        *k = kk.clone();
                    }
                    if let Some(vv) = right_clone.get_val() {
                        *v = vv.clone();
                    }
                }
                // Balance 4-node
                // Rotate Right
                if left.is_red() && left.is_left_red() {
                    let left_clone = left.clone();
                    *left = left_clone.get_left_clone();
                    let left_size = left.size();
                    *color = true;
                    right.set_vals(
                        &k,
                        &v,
                        true,
                        left_size,
                        *left_clone.get_right_clone(),
                        *right.clone(),
                    );
                    // Don't move, but use clone, instead, from left clone
                    if let Some(kk) = left_clone.get_key() {
                        *k = kk.clone();
                    }
                    if let Some(vv) = left_clone.get_val() {
                        *v = vv.clone();
                    }
                }
                // Split 4-node
                // Flip colors
                if left.is_red() && right.is_red() {
                    *color = true;
                    left.set_color(false);
                    right.set_color(false);
                }
                *size = left.size() + right.size() + 1_usize;
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

    fn get_height(&self) -> usize {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                ref left,
                ref right,
            } => 1_usize + std::cmp::max(left.get_height(), right.get_height()),
            _ => 0_usize,
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

    fn get_key(&self) -> Option<&K> {
        if let RedBlackTree::Node {
            ref k,
            v: _,
            color: _,
            size: _,
            left: _,
            right: _,
        } = self
        {
            Some(k)
        } else {
            None
        }
    }

    fn get_val(&self) -> Option<&V> {
        if let RedBlackTree::Node {
            k: _,
            ref v,
            color: _,
            size: _,
            left: _,
            right: _,
        } = self
        {
            Some(v)
        } else {
            None
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

    fn is_right_red(&self) -> bool {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                left: _,
                ref right,
            } => right.is_red(),
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
    use crate::{SedgewickMap, Traversals, TreeTraversal};

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
        assert_eq!(rbtree.size(), 0_usize);
        assert_eq!(rbtree.height(), None);
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
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::InOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_random_pre_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['c', 'b', 'a', 'd'];
        let mut it = res.iter();
        rbtree.put('c', 3);
        rbtree.put('d', 4);
        rbtree.put('b', 2);
        rbtree.put('a', 1);
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::PreOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_random_post_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['a', 'b', 'd', 'c'];
        let mut it = res.iter();
        rbtree.put('c', 3);
        rbtree.put('d', 4);
        rbtree.put('b', 2);
        rbtree.put('a', 1);
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::PostOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_random_level_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['c', 'b', 'd', 'a'];
        let mut it = res.iter();
        rbtree.put('c', 3);
        rbtree.put('d', 4);
        rbtree.put('b', 2);
        rbtree.put('a', 1);
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::LevelOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_left_rotate_size_and_height() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let mut i = 1;
        for c in 'a'..='i' {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), 9_usize);
        assert_eq!(rbtree.height(), Some(3_usize));
    }

    #[test]
    fn test_left_rotate_pre_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['d', 'b', 'a', 'c', 'h', 'f', 'e', 'g', 'i'];
        let mut it = res.iter();
        let mut i = 1;
        for c in 'a'..='i' {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::PreOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_left_rotate_in_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
        let mut it = res.iter();
        let mut i = 1;
        for c in 'a'..='i' {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::InOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_left_rotate_post_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['a', 'c', 'b', 'e', 'g', 'f', 'i', 'h', 'd'];
        let mut it = res.iter();
        let mut i = 1;
        for c in 'a'..='i' {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::PostOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_left_rotate_level_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['d', 'b', 'h', 'a', 'c', 'f', 'i', 'e', 'g'];
        let mut it = res.iter();
        let mut i = 1;
        for c in 'a'..='i' {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::LevelOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_right_rotate_size_and_height() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let mut i = 1;
        for c in ('a'..='i').rev() {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), 9_usize);
        assert_eq!(rbtree.height(), Some(3_usize));
    }

    #[test]
    fn test_right_rotate_pre_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['f', 'd', 'b', 'a', 'c', 'e', 'h', 'g', 'i'];
        let mut it = res.iter();
        let mut i = 1;
        for c in ('a'..='i').rev() {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::PreOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_right_rotate_in_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
        let mut it = res.iter();
        let mut i = 1;
        for c in ('a'..='i').rev() {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::InOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_right_rotate_post_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['a', 'c', 'b', 'e', 'd', 'g', 'i', 'h', 'f'];
        let mut it = res.iter();
        let mut i = 1;
        for c in ('a'..='i').rev() {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::PostOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_right_rotate_level_order() {
        let mut rbtree: RedBlackTree<char, i32> = RedBlackTree::new();
        let res = vec!['f', 'd', 'h', 'b', 'e', 'g', 'i', 'a', 'c'];
        let mut it = res.iter();
        let mut i = 1;
        for c in ('a'..='i').rev() {
            rbtree.put(c, i);
            i += 1;
        }
        assert_eq!(rbtree.size(), res.len());
        for (a, _) in rbtree.traverse(&Traversals::LevelOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_left_rotate_thousand() {
        let mut rbtree = RedBlackTree::new();
        for i in 1..=1_000_u32 {
            rbtree.put(i, i);
        }
        let cnt = rbtree.traverse(&Traversals::InOrder).count();
        assert_eq!(cnt, 1_000_usize);
        assert_eq!(rbtree.size(), 1_000_usize);
        assert_eq!(rbtree.size(), cnt);
        assert_eq!(rbtree.height(), Some(9_usize));
        assert_eq!(rbtree.min(), Some(&1_u32));
        assert_eq!(rbtree.max(), Some(&1000_u32));
        assert_eq!(rbtree.get(&501_u32), Some(&501_u32));
    }

    #[test]
    fn test_right_rotate_thousand() {
        let mut rbtree = RedBlackTree::new();
        for i in (1..=1_000_u32).rev() {
            rbtree.put(i, i);
        }
        let cnt = rbtree.traverse(&Traversals::InOrder).count();
        assert_eq!(cnt, 1_000_usize);
        assert_eq!(rbtree.size(), 1_000_usize);
        assert_eq!(rbtree.size(), cnt);
        assert_eq!(rbtree.height(), Some(14_usize));
        assert_eq!(rbtree.min(), Some(&1_u32));
        assert_eq!(rbtree.max(), Some(&1000_u32));
        assert_eq!(rbtree.get(&501_u32), Some(&501_u32));
    }
}
