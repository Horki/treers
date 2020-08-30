use crate::SedgewickMap;
use std::ops::Index;

// TODO: add M size in constructor?
const M: usize = 4_usize;

// TODO: make stack memory array
type Node<K, V> = Vec<Entry<K, V>>;

#[derive(Debug)]
struct Entry<K: Ord + Clone, V: Clone> {
    key: K,
    val: Option<V>,
    next: Node<K, V>,
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
    fn new(key: K, val: Option<V>) -> Self {
        Self {
            key,
            val,
            next: Vec::with_capacity(M),
        }
    }
    fn create(key: K, val: Option<V>, next: Node<K, V>) -> Self {
        Self { key, val, next }
    }
}

/// 3.3 Balanced Tree
///
/// BTree implementation from Robert Sedgewick book, "Algorithms" 4th edition
///
/// # Examples
///
/// ```
/// use treers::btree::BalancedTree;
/// use treers::SedgewickMap;
///
/// let mut btree: BalancedTree<char, i32> = BalancedTree::new();
/// btree.put('c', 3);
/// btree.put('d', 4);
/// btree.put('b', 2);
/// btree.put('a', 1);
///
/// // Generate a Balanced Tree, M = 4
/// //    [ a  c ]
/// //      |  |
/// //      b  d
///
/// // Gets a value 1
/// println!("bst[a] = {}", btree.get(&'a').unwrap());
/// assert_eq!(btree.height(), Some(1_usize));
/// ```
#[derive(Debug)]
pub struct BalancedTree<K: Ord + Clone, V: Clone> {
    root: Node<K, V>,
    size: usize,
    height: usize,
}

impl<K: Ord + Clone, V: Clone> SedgewickMap<K, V> for BalancedTree<K, V> {
    /// Inits a new instance of Balanced Tree.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::btree::BalancedTree;
    /// use treers::SedgewickMap;
    ///
    /// let btree: BalancedTree<char, i32> = BalancedTree::new();
    /// assert_eq!(btree.is_empty(), true);
    /// ```
    fn new() -> Self {
        Self {
            root: Vec::with_capacity(M),
            size: 0_usize,
            height: 0_usize,
        }
    }

    /// Returns a size of elements in `BST`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::btree::BalancedTree;
    /// use treers::SedgewickMap;
    ///
    /// let mut btree: BalancedTree<char, i32> = BalancedTree::new();
    /// assert_eq!(btree.size(), 0_usize);
    /// btree.put('a', 1);
    /// btree.put('b', 2);
    /// btree.put('c', 3);
    /// btree.put('d', 4);
    /// assert_eq!(btree.size(), 4_usize);
    /// ```
    fn size(&self) -> usize {
        self.size
    }

    // TODO: fix lifetime params
    /// Returns a reference to optional reference to value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::btree::BalancedTree;
    /// use treers::SedgewickMap;
    ///
    /// let mut btree: BalancedTree<char, i32> = BalancedTree::new();
    /// btree.put('a', 1);
    /// assert_eq!(btree.get(&'a'), Some(&1));
    /// assert_eq!(btree.get(&'b'), None);
    /// assert_eq!(btree[&'a'], 1);
    /// ```
    fn get(&self, key: &K) -> Option<&V> {
        if self.is_empty() {
            None
        } else {
            search(&self.root, key.clone(), self.height)
        }
    }

    /// Insert a key-value pair into the `BTree`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::btree::BalancedTree;
    /// use treers::SedgewickMap;
    ///
    /// let mut btree: BalancedTree<char, i32> = BalancedTree::new();
    /// assert_eq!(btree.is_empty(), true);
    ///
    /// btree.put('a', 1_i32);
    /// assert_eq!(btree.is_empty(), false);
    /// assert_eq!(btree.get(&'a'), Some(&1_i32));
    /// assert_eq!(btree[&'a'], 1_i32);
    /// ```
    fn put(&mut self, key: K, value: V) {
        if let Some(u) = insert(&mut self.root, key, value, self.height) {
            // need to split the root
            let mut t: Node<K, V> = Vec::with_capacity(M / 2);
            t.push(Entry::create(
                self.root[0].key.clone(),
                None,
                self.root.clone(),
            ));
            t.push(Entry::create(u[0].key.clone(), None, u));
            self.root = t;
            self.height += 1;
        }
        self.size += 1;
    }

    /// Get height of `BTree`.
    ///
    /// BTree is balanced tree. TODO: add more text
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::btree::BalancedTree;
    /// use treers::SedgewickMap;
    ///
    /// let mut btree: BalancedTree<char, i32> = BalancedTree::new();
    /// assert_eq!(btree.height(), Some(0_usize));
    /// btree.put('a', 1);
    /// btree.put('b', 2);
    /// btree.put('c', 3);
    /// btree.put('d', 4);
    /// btree.put('e', 5);
    /// btree.put('f', 6);
    /// btree.put('g', 7);
    /// //    |a|c|e|         <-- height: 0
    /// //    /  |  \
    /// // |b|  |d|  |f|g|    <-- height: 1
    /// //
    /// // Note -The Height of balanced tree with single node is taken as zero,
    /// //       but empty BTree is 0, not None.
    /// assert_eq!(btree.height(), Some(1_usize));
    /// assert_eq!(btree.get(&'g'), Some(&7_i32));
    /// assert_eq!(btree[&'g'], 7_i32);
    /// assert_eq!(btree.size(), 7_usize);
    /// ```
    fn height(&self) -> Option<usize> {
        Some(self.height)
    }

    /// Returns a optional reference to minimal key
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::btree::BalancedTree;
    /// use treers::SedgewickMap;
    ///
    /// let mut btree: BalancedTree<char, i32> = BalancedTree::new();
    /// assert_eq!(btree.min(), None);
    /// btree.put('c', 1);
    /// btree.put('a', 2);
    /// btree.put('b', 3);
    /// btree.put('d', 4);
    /// assert_eq!(btree.min(), Some(&'a'));
    /// ```
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

    /// Returns a optional reference to maximum key
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use treers::btree::BalancedTree;
    /// use treers::SedgewickMap;
    ///
    /// let mut btree: BalancedTree<char, i32> = BalancedTree::new();
    /// assert_eq!(btree.max(), None);
    /// btree.put('c', 1);
    /// btree.put('a', 2);
    /// btree.put('b', 3);
    /// btree.put('d', 4);
    /// assert_eq!(btree.max(), Some(&'d'));
    /// ```
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

// TODO: fix lifetime params for search!
fn search<'a, K, V>(node: &'a [Entry<K, V>], key: K, height: usize) -> Option<&'a V>
where
    K: Ord + Clone + 'a,
    V: Clone + 'a,
{
    if height.eq(&0_usize) {
        for n in node {
            if key.eq(&n.key) {
                return n.val.as_ref();
            }
        }
    } else {
        for j in 0..node.len() {
            if (j + 1).eq(&node.len()) || key.lt(&node[j + 1].key) {
                return search(&node[j].next, key, height - 1_usize);
            }
        }
    }
    None
}

fn insert<K, V>(h: &mut Node<K, V>, key: K, val: V, height: usize) -> Option<Node<K, V>>
where
    K: Ord + Clone,
    V: Clone,
{
    let mut j = 0;
    let mut t = Entry::new(key.clone(), Some(val.clone()));
    if height == 0_usize {
        // External Node
        while j < h.len() {
            if key.lt(&h[j].key) {
                break;
            }
            j += 1;
        }
    } else {
        // Internal Node
        while j < h.len() {
            if (j + 1_usize).eq(&h.len()) || key.lt(&h[j + 1].key) {
                if let Some(u) = insert(&mut h[j].next, key, val, height - 1_usize) {
                    t.key = u[0].key.clone();
                    t.val = None;
                    t.next = u;
                    j += 1;
                    break;
                } else {
                    return None;
                }
            }
            j += 1;
        }
    }
    let mut i = h.len();
    while i.gt(&j) {
        if i.eq(&h.len()) {
            h.push(h[i - 1].clone());
        } else {
            h.swap(i, i - 1);
        }
        i -= 1;
    }
    if j.eq(&h.len()) {
        h.push(t);
    } else {
        h[j] = t;
    }

    if h.len().lt(&M) {
        None
    } else {
        // Split node in half
        let mut t: Node<K, V> = Vec::with_capacity(M / 2);
        // TODO: work for M=4, find a better solution!
        for _ in 0..(M / 2) {
            t.push(h.remove(M / 2));
        }
        Some(t)
    }
}

impl<K: Ord + Clone, V: Clone> Default for BalancedTree<K, V> {
    /// Creates an empty `BalancedTree<K, V>`.
    fn default() -> BalancedTree<K, V> {
        BalancedTree::new()
    }
}

impl<K: Ord + Clone, V: Clone> Index<&K> for BalancedTree<K, V> {
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `BalancedTree`.
    #[inline]
    fn index(&self, index: &K) -> &V {
        self.get(index)
            .expect("Missing entry for key in Balanced Tree")
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
        assert_eq!(btree.height(), Some(0));
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
        assert_eq!(btree.get(&10_u32), None);
        assert_eq!(btree[&1_u32], -1_i32);
    }

    #[test]
    #[should_panic]
    fn test_index_panic() {
        let mut btree: BalancedTree<i32, i32> = BalancedTree::new();
        btree.put(1_i32, -1_i32);
        let _panics = btree[&10_i32];
    }

    #[test]
    fn test_contains() {
        let mut btree: BalancedTree<i32, i32> = BalancedTree::new();
        btree.put(1_i32, -1_i32);
        assert_eq!(btree.contains(&1_i32), true);
        assert_eq!(btree.contains(&-1_i32), false);
    }

    #[test]
    fn test_left_rotate_one_thousand() {
        let mut btree: BalancedTree<i32, i32> = BalancedTree::new();
        assert_eq!(btree.min(), None);
        assert_eq!(btree.max(), None);
        for i in 1..=1_000_i32 {
            btree.put(i, i + 1);
        }
        assert_eq!(btree.size(), 1_000_usize);
        assert_eq!(btree.height(), Some(8_usize));
        assert_eq!(btree.min(), Some(&1_i32));
        assert_eq!(btree.max(), Some(&1_000_i32));
        assert_eq!(btree.get(&501_i32), Some(&502_i32));
        assert_eq!(btree.contains(&501_i32), true);
    }

    #[test]
    fn test_right_rotate_one_thousand() {
        let mut btree: BalancedTree<i32, i32> = BalancedTree::new();
        for i in (1..=1_000_i32).rev() {
            btree.put(i, i + 1);
        }
        assert_eq!(btree.size(), 1_000_usize);
        assert_eq!(btree.height(), Some(8_usize));
        assert_eq!(btree.min(), Some(&1_i32));
        assert_eq!(btree.max(), Some(&1_000_i32));
        assert_eq!(btree.get(&501_i32), Some(&502_i32));
        assert_eq!(btree.contains(&501_i32), true);
    }
}
