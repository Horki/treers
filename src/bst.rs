use super::{SedgewickMap, Traversals};
use std::cmp::Ordering;

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
    fn new() -> Self {
        BST::NIL
    }

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

    fn height(&self) -> usize {
        match self {
            BST::Node {
                k: _,
                v: _,
                size: _,
                ref left,
                ref right,
            } => 1_usize + std::cmp::max(left.height(), right.height()),
            _ => 0_usize,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            BST::Node { .. } => false,
            _ => true,
        }
    }

    fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

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
}

impl<'a, K: 'a + Ord, V: 'a> BST<K, V> {
    fn in_order(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
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
    fn pre_order(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
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

    fn post_order(&'a self, vec: &mut Vec<(&'a K, &'a V)>) {
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

    fn level_order(&'a self, vec: &mut Vec<(&'a K, &'a V)>, level: usize) {
        if let BST::Node {
            ref k,
            ref v,
            size: _,
            ref left,
            ref right,
        } = self
        {
            if level == 1 {
                vec.push((k, v));
            } else if level > 1 {
                left.level_order(vec, level - 1);
                right.level_order(vec, level - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SedgewickMap, Traversals, BST};

    #[test]
    fn test_is_empty() {
        let bst: BST<i32, i32> = BST::new();
        println!("{}", std::mem::size_of_val(&bst));
        assert_eq!(bst.is_empty(), true);
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
        assert_eq!(bst.height(), 0);
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
    }

    #[test]
    fn test_contains() {
        let mut bst: BST<i32, i32> = BST::new();
        bst.put(1_i32, -1_i32);
        assert_eq!(bst.contains(&1_i32), true);
        assert_eq!(bst.contains(&-1_i32), false);
    }

    #[test]
    fn test_size_two() {
        let mut bst: BST<i32, i32> = BST::new();
        bst.put(1, 2);
        bst.put(2, 3);
        assert_eq!(bst.size(), 2);
    }

    // #[should_panic]
    #[test]
    #[ignore]
    fn ignored_test_size_million() {
        let mut bst: BST<u64, u64> = BST::new();
        for i in 1..=1_000_001_u64 {
            bst.put(i, i + 1);
        }
        assert_eq!(bst.size(), 1_000_000);
    }

    #[test]
    fn test_size_and_height_one_thousand() {
        let mut bst: BST<u64, u64> = BST::new();
        for i in 1..=1_000_u64 {
            bst.put(i, i + 1);
        }
        assert_eq!(bst.size(), 1_000);
        assert_eq!(bst.height(), 1_000);
    }

    #[test]
    fn test_min() {
        let mut bst: BST<u32, u32> = BST::new();
        for i in vec![6_u32, 4, 5, 2, 1, 3] {
            bst.put(i, i);
        }
        assert_eq!(bst.min(), Some(&1_u32));
        assert_eq!(bst.get(bst.min().unwrap()), bst.min());
    }

    #[test]
    fn test_max() {
        let mut bst: BST<u32, u32> = BST::new();
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
        for (a, _) in bst.traverse(&Traversals::PostOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }

    #[test]
    fn test_level_order() {
        let mut bst: BST<char, i32> = BST::new();
        let res = vec!['c', 'b', 'd', 'a'];
        let mut it = res.iter();
        bst.put('c', 3);
        bst.put('d', 4);
        bst.put('b', 2);
        bst.put('a', 1);
        for (a, _) in bst.traverse(&Traversals::LevelOrder) {
            assert_eq!(*a, *it.next().unwrap());
        }
    }
}
