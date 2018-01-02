
use std::cmp::Ordering;
use std::fmt::Display;

impl<K: Ord, V> Tree<K, V> {

    pub fn new() -> Tree<K, V> {
        return Tree { root: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        let result = Tree::insert_node(&mut self.root, key, value);
        if !result {
            self.size += 1;
        }
        return result;
    }

    pub fn get(self, key: K) -> Option<V> {
        return Tree::get_value(self.root, key);        
    }

    pub fn lower_bound(&self, key: K) -> Option<(K, V)> {
        return Tree::find_lower(&self.root, key);
    }

    fn find_lower(node: &Option<Box<Node<K, V>>>, key: K) -> Option<(K, V)> {
        return None;
    }

    fn get_value(node: Option<Box<Node<K, V>>>, key: K) -> Option<V> {
        return node.and_then(|n| {
            match key.cmp(&n.key) {
                Ordering::Less => return Tree::get_value(n.left, key),
                Ordering::Greater => return Tree::get_value(n.right, key),
                Ordering::Equal => return Some(n.value)
            }
        });
    }

    fn insert_node(node: &mut Option<Box<Node<K, V>>>, key: K, value: V) -> bool {
        match *node {
            None => {
                *node = Some(Box::new(Node { key: key, value: value, left: None, right: None }));
                return false;
            },
            Some(ref mut node) => {
                match key.cmp(&node.key) {
                    Ordering::Less => {
                        return Tree::insert_node(&mut node.left, key, value);
                    },
                    Ordering::Greater => {
                        return Tree::insert_node(&mut node.right, key, value);
                    },
                    Ordering::Equal => {
                        node.key = key;
                        node.value = value;
                        return true;
                    },
                }
            },
        }
    }
}

#[derive(Clone)]
pub struct Tree<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
    size: usize,
}

#[derive(Clone)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_size() {
        let tree: Tree<i32, i32> = Tree::new();
        assert_eq!(0, tree.size(), "correct size for empty trees");
    }

    #[test]
    fn get_empty_tree() {
        let tree: Tree<String, String> = Tree::new();
        assert_eq!(None, tree.get("key".to_string()), "nothing should be in the tree");
    }

    #[test]
    fn insert_one() {
        let mut tree: Tree<i32, i32> = Tree::new();
        tree.insert(1, 2);
        assert_eq!(1, tree.size(), "size of one after insert");
    }

    #[test]
    fn insert_lots() {
        let size = 100;
        let mut tree: Tree<i32, i32> = Tree::new();

        for i in 0..size {
            tree.insert(i, i);
        }
        assert_eq!(size as usize, tree.size(), "large size");
    }

    #[test]
    fn duplicate_insert() {
        let mut tree: Tree<i32, i32> = Tree::new();

        for i in 0..5 {
            tree.insert(10, 10);
        }

        assert_eq!(1, tree.size(), "handles duplicates");
    }

    #[test]
    fn insert_and_get() {
        let mut tree: Tree<i32, String> = Tree::new();

        tree.insert(1, "one".to_string());

        assert_eq!(Some("one".to_string()), tree.get(1), "should return inserted value");
    }

    #[test]
    fn find_lower_exact_value() {
        let mut tree: Tree<i32, i32> = Tree::new();

        tree.insert(1, 1);
        tree.insert(5, 5);
        tree.insert(10, 10);

        let result = tree.lower_bound(5);
        assert_eq!(Some((5, 5)), result);
    }

    #[test]
    fn find_lower() {
        let mut tree: Tree<i32, i32> = Tree::new();

        tree.insert(1, 1);
        tree.insert(5, 5);
        tree.insert(10, 10);

        assert_eq!(Some((5, 5)), tree.lower_bound(6));
    }

    #[test]
    fn find_lower_2() {
        let mut tree: Tree<i32, i32> = Tree::new();

        tree.insert(5, 5);
        tree.insert(1, 1);
        tree.insert(10, 10);

        assert_eq!(Some((1, 1)), tree.lower_bound(2));
    }

    #[test]
    fn find_lower_3() {
        let mut tree: Tree<i32, i32> = Tree::new();

        tree.insert(10, 10);
        tree.insert(5, 5);
        tree.insert(1, 1);

        assert_eq!(Some((1, 1)), tree.lower_bound(2));
    }

    #[test]
    fn no_lower_bound() {
        let mut tree: Tree<i32, i32> = Tree::new();
        tree.insert(5, 5);
        tree.insert(1, 1);
        tree.insert(10, 10);
        assert_eq!(None, tree.lower_bound(0));
    }

    #[test]
    fn find_lower_4() {
        let mut tree: Tree<i32, i32> = Tree::new();
        tree.insert(50, 50);
        tree.insert(25, 25);
        tree.insert(12, 12);
        tree.insert(37, 37);
        tree.insert(75, 75);
        tree.insert(60, 60);
        tree.insert(100, 100);

        let r1 = tree.lower_bound(52);
        let r2 = tree.lower_bound(99);
        
        assert_eq!(Some((50, 50)), tree.lower_bound(52));
        assert_eq!(Some((75, 75)), tree.lower_bound(99));
    }
        
}
