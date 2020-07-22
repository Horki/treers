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

    // TODO: implement later
    fn traverse(&self, _traverse: &Traversals) -> std::vec::IntoIter<(&K, &V)> {
        unimplemented!()
    }
}

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
        } = self {
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
    use crate::SedgewickMap;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn is_empty() {
        let r: RedBlackTree<i32, i32> = RedBlackTree::new();
        assert_eq!(r.is_empty(), true);
    }
}
