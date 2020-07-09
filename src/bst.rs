use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum BST {
    Node {
        k: i32,
        v: i32,
        size: usize,
        left: Box<BST>,
        right: Box<BST>,
    },
    NIL,
}

impl BST {
    pub fn new() -> Self {
        BST::NIL
    }
    pub fn size(&self) -> usize {
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
    pub fn get(&self, key: i32) -> Option<i32> {
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
                _ => Some(*v),
            },
            _ => None,
        }
    }
    fn insert(&mut self, key: i32, value: i32) {
        match self {
            BST::Node {
                ref mut k,
                ref mut v,
                ref mut size,
                ref mut left,
                ref mut right,
            } => match key.cmp(k) {
                Ordering::Less => left.insert(key, value),
                Ordering::Greater => right.insert(key, value),
                _ => {}
            },
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

    pub fn put(&mut self, key: i32, value: i32) {
        self.insert(key, value);
    }

    pub fn height(&self) -> usize {
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
    pub fn is_empty(&self) -> bool {
        match self {
            BST::Node { .. } => false,
            _ => true,
        }
    }
    pub fn contains(&self, key: i32) -> bool {
        self.get(key).is_some()
    }
    pub fn min(&self) -> Option<i32> {
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
                    Some(*k)
                }
            }
            _ => None,
        }
    }

    pub fn max(&self) -> Option<i32> {
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
                    Some(*k)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bst::BST;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn is_empty() {
        let bst = BST::new();
        assert_eq!(bst.is_empty(), true);
    }
}
