use std::cmp::Ordering;

#[derive(Debug)]
enum RedBlackTree {
    Node {
        k: i32,
        v: i32,
        color: bool,
        size: usize,
        left: Box<RedBlackTree>,
        right: Box<RedBlackTree>,
    },
    NIL,
}

impl RedBlackTree {
    pub fn new() -> Self {
        RedBlackTree::NIL
    }
    pub fn size(&self) -> usize {
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
    pub fn get(&self, key: i32) -> Option<i32> {
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
                _ => Some(*v),
            },
            _ => None,
        }
    }
    fn insert(&mut self, key: i32, value: i32) {
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
                    Ordering::Less => left.insert(key, value),
                    Ordering::Greater => right.insert(key, value),
                    _ => {}
                }
                // Lean left
                println!("inserting numero {}", k);
                if right.is_red() && !left.is_red() {
                    println!("rotate left {}", k);

                    // left.set_vals(*k, *v, *color, *size, RedBlackTree::NIL, RedBlackTree::NIL);
                    // if right.left.contains(key) {
                    //     *right = Box::new(RedBlackTree::NIL);
                    // }
                    // *k = key;
                    // *v = value;
                    // *color = true;
                    // *size = left.size() + right.size() + 1;
                }
                // Balance 4-node
                if left.is_red() {
                    if let Some(l) = left.get_left() {
                        if l.is_red() {
                            println!("rotate right {}", k);
                        }
                    }
                    // std::mem::swap(left, right);
                    // right.set_vals(*k, *v, *color);
                    // *k = key;
                    // *v = value;
                    // *color = true;
                    // *size = left.size() + right.size() + 1;
                }
                // Split 4-node
                if left.is_red() && right.is_red() {
                    *color = true;
                    left.set_black();
                    right.set_black();
                }
                *size = left.size() + right.size() + 1;
            }
            RedBlackTree::NIL => {
                // Insert a leaf node
                *self = RedBlackTree::Node {
                    k: key,
                    v: value,
                    color: true,
                    size: 1,
                    left: Box::new(RedBlackTree::NIL),
                    right: Box::new(RedBlackTree::NIL),
                }
            }
        }
    }

    fn remove_children(&mut self) {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                ref mut left,
                ref mut right,
            } => {
                *left = Box::new(RedBlackTree::NIL);
                *right = Box::new(RedBlackTree::NIL);
            }
            _ => {}
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.insert(key, value);
        // set root node to black
        self.set_black();
    }

    fn set_vals(
        &mut self,
        key: i32,
        val: i32,
        c: bool,
        s: usize,
        l: RedBlackTree,
        r: RedBlackTree,
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
                *k = key;
                *v = val;
                *color = c;
                *size = s;
                *left = Box::new(r);
                *right = Box::new(l);
            }
            RedBlackTree::NIL => {
                *self = RedBlackTree::Node {
                    k: key,
                    v: val,
                    color: c,
                    size: 1,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
        }
    }

    fn set_black(&mut self) {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                ref mut color,
                size: _,
                left: _,
                right: _,
            } => *color = false,
            _ => {}
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

    fn get_left(&self) -> Option<&RedBlackTree> {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                ref left,
                right: _,
            } => Some(left),
            _ => None,
        }
    }

    fn get_left_mut(&mut self) -> Option<&mut RedBlackTree> {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                ref mut left,
                right: _,
            } => Some(left),
            _ => None,
        }
    }

    fn get_right(&self) -> Option<&RedBlackTree> {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                left: _,
                ref right,
            } => Some(right),
            _ => None,
        }
    }

    fn get_right_mut(&mut self) -> Option<&mut RedBlackTree> {
        match self {
            RedBlackTree::Node {
                k: _,
                v: _,
                color: _,
                size: _,
                left: _,
                ref mut right,
            } => Some(right),
            _ => None,
        }
    }

    pub fn height(&self) -> usize {
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
    pub fn is_empty(&self) -> bool {
        match self {
            RedBlackTree::Node { .. } => false,
            _ => true,
        }
    }
    pub fn contains(&self, key: i32) -> bool {
        self.get(key).is_some()
    }
    pub fn min(&self) -> Option<i32> {
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
                    Some(*k)
                }
            }
            _ => None,
        }
    }

    pub fn max(&self) -> Option<i32> {
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
                    Some(*k)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rbtree::RedBlackTree;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn is_empty() {
        let r = RedBlackTree::new();
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    fn test_main() {
        let mut tree = RedBlackTree::new();
        tree.put(1, 2);
        println!("{:?}", tree);
        tree.put(2, 4);
        println!("{:?}", tree);
        tree.put(3, 4);
        println!("{:?}", tree);
    }
}
