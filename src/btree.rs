struct BTreeNode {
    key: i32,
    value: i32,
}

impl BTreeNode {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::btree::BTreeNode;

    #[test]
    fn test_node() {
        let node = BTreeNode::new(1, 42);
        assert_eq!(node.key, 1);
        assert_eq!(node.value, 42);
    }
}