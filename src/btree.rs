use crate::SedgewickMap;

const M: usize = 4_usize;

#[derive(Debug)]
pub struct Entry<K: Ord + Clone, V: Clone> {
    key: K,
    val: V,
    next: Vec<Entry<K, V>>,
}

impl<K: Clone + Ord, V: Clone> Clone for Entry<K, V> {
    fn clone(&self) -> Entry<K, V> {
        Entry {
            key: self.key.clone(),
            val: self.val.clone(),
            next: self.next.clone(),
        }
    }
}

impl<K: Ord + Clone, V: Clone> Entry<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            next: Vec::with_capacity(M),
        }
    }
    fn create(key: K, val: V, next: Vec<Entry<K, V>>) -> Self {
        Self { key, val, next }
    }
}

#[derive(Debug)]
pub struct BalancedTree<K: Ord + Clone, V: Clone> {
    root: Vec<Entry<K, V>>,
    size: usize,
    height: usize,
}

impl<K: Ord + Clone, V: Clone> SedgewickMap<K, V> for BalancedTree<K, V> {
    fn new() -> Self {
        Self {
            root: Vec::with_capacity(M),
            size: 0_usize,
            height: 0_usize,
        }
    }
    fn size(&self) -> usize {
        self.size
    }

    // TODO: fix lifetime params
    fn get(&self, key: &K) -> Option<&V> {
        if self.is_empty() {
            None
        } else {
            Self::search(&self.root, key.clone(), self.height)
        }
    }

    fn put(&mut self, key: K, value: V) {
        if let Some(u) = Self::insert(&mut self.root, key, value, self.height) {
            // need to split the root
            let mut t: Vec<Entry<K, V>> = Vec::with_capacity(M / 2);
            t.push(Entry::create(
                self.root[0].key.clone(),
                self.root[0].val.clone(),
                self.root.clone(),
            ));
            t.push(Entry::create(u[0].key.clone(), u[0].val.clone(), u));
            self.root = t;
            self.height += 1;
        }
        self.size += 1;
    }

    fn height(&self) -> usize {
        self.height
    }

    fn min(&self) -> Option<&K> {
        if self.is_empty() {
            return None;
        }
        let mut node = &self.root;
        loop {
            let next = &node[0].next;
            if !next.is_empty() {
                node = &next;
            } else {
                return Some(&node[0].key);
            }
        }
    }

    fn max(&self) -> Option<&K> {
        if self.is_empty() {
            return None;
        }
        let mut node = &self.root;
        loop {
            let next = &node[node.len() - 1].next;
            if !next.is_empty() {
                node = &next;
            } else {
                return Some(&node[node.len() - 1].key);
            }
        }
    }
}

impl<'a, K: Ord + Clone + 'a, V: Clone + 'a> BalancedTree<K, V> {
    // TODO: fix lifetime params for search!
    fn search(node: &'a [Entry<K, V>], key: K, height: usize) -> Option<&'a V> {
        if height == 0_usize {
            for n in node {
                if key.eq(&n.key) {
                    return Some(&n.val);
                }
            }
        } else {
            for j in 0..node.len() {
                if (j + 1).eq(&node.len()) || key.lt(&node[j + 1].key) {
                    return Self::search(&node[j].next, key, height - 1);
                }
            }
        }
        None
    }
    // TODO: fix lifetime params for insert!
    fn insert(h: &mut Vec<Entry<K, V>>, key: K, val: V, height: usize) -> Option<Vec<Entry<K, V>>> {
        let mut j = 0;
        let mut t = Entry::new(key.clone(), val.clone());
        if height == 0_usize {
            // External Node
            // for j in 0..h.len() {
            while j < h.len() {
                if key.lt(&h[j].key) {
                    break;
                }
                j += 1;
            }
        } else {
            j = 0;
            // Internal Node
            // for j in 0..h.len() {
            while j < h.len() {
                if (j + 1).eq(&h.len()) || key.lt(&h[j + 1].key) {
                    if let Some(u) = Self::insert(&mut h[j].next, key, val, height - 1_usize) {
                        t.key = u[0].key.clone();
                        t.next = u;
                        break;
                    } else {
                        return None;
                    }
                }
                j += 1;
            }
        }
        for i in (j..h.len()).rev() {
            h[i] = h[i - 1].clone();
        }
        h.insert(j, t);

        if h.len().lt(&M) {
            None
        } else {
            // Split node in half
            let mut t: Vec<Entry<K, V>> = Vec::with_capacity(M / 2);
            for j in 0..(M / 2) {
                t.push(h[M / 2 + j].clone());
            }
            Some(t)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::btree::BalancedTree;
    use crate::SedgewickMap;

    #[test]
    fn test_is_empty() {
        let btree: BalancedTree<i32, i32> = BalancedTree::new();
        assert_eq!(btree.is_empty(), true);
    }

    #[test]
    fn test_is_not_empty() {
        let mut btree = BalancedTree::new();
        btree.put(1, 2);
        btree.put(2, 4);
        assert_eq!(btree.is_empty(), false);
    }

    #[test]
    fn test_size_zero() {
        let btree: BalancedTree<i32, i32> = BalancedTree::new();
        assert_eq!(btree.size(), 0_usize);
        assert_eq!(btree.height(), 0_usize);
    }

    #[test]
    fn test_put() {
        let mut btree: BalancedTree<u32, Vec<i32>> = BalancedTree::new();
        let v = vec![1, 2, 3];
        btree.put(1, v);
        assert_eq!(btree.get(&1_u32), Some(&vec![1_i32, 2, 3]));
    }

    #[test]
    fn test_get() {
        let mut btree: BalancedTree<u32, i32> = BalancedTree::new();
        btree.put(1_u32, -1_i32);
        assert_eq!(btree.get(&1_u32), Some(&-1_i32));
    }

    #[test]
    fn test_contains() {
        let mut btree: BalancedTree<i32, i32> = BalancedTree::new();
        btree.put(1_i32, -1_i32);
        assert_eq!(btree.contains(&1_i32), true);
        assert_eq!(btree.contains(&-1_i32), false);
    }

    #[test]
    #[ignore]
    fn test_left_rotate_one_thousand() {
        // TODO: fix 'search is not working properly'
        let mut btree: BalancedTree<i32, i32> = BalancedTree::new();
        for i in 1..=1_000 {
            btree.put(i, i + 1);
        }
        assert_eq!(btree.size(), 1_000_usize);
        assert_eq!(btree.height(), 9_usize);
        assert_eq!(btree.min(), Some(&1_i32));
        assert_eq!(btree.max(), Some(&1_000_i32));
        assert_eq!(btree.get(&501_i32), Some(&502_i32));
        assert_eq!(btree.contains(&501_i32), true);
    }

    #[test]
    #[ignore]
    fn test_right_rotate_one_thousand() {
        // TODO: fix 'attempt to subtract with overflow'
        let mut btree: BalancedTree<i32, i32> = BalancedTree::new();
        for i in (1..=1_000).rev() {
            btree.put(i, i + 1);
        }
        assert_eq!(btree.size(), 1_000_usize);
        assert_eq!(btree.height(), 9_usize);
        assert_eq!(btree.min(), Some(&1_i32));
        assert_eq!(btree.max(), Some(&1_000_i32));
        assert_eq!(btree.get(&501_i32), Some(&502_i32));
        assert_eq!(btree.contains(&501_i32), true);
    }
}
