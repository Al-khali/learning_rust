use std::fmt::Debug;

fn main() {
    let mut bst = Bst::new();
    bst.insert(3_i32);
    bst.insert(4);
    bst.insert(1);
    bst.insert(12);
    bst.insert(44);
    bst.insert(33);
    bst.insert(50);
    bst.insert(0);
    println!("{:#?}", bst);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bst<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Default for Bst<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T> Bst<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_value(val: T) -> Self {
        let root = Box::new(Node {
            val,
            left: None,
            right: None,
        });
        Self { root: Some(root) }
    }
}

impl<T> Bst<T>
where
    T: Ord,
{
    pub fn insert(&mut self, new_val: T) {
        let new_node = Box::new(Node {
            val: new_val,
            left: None,
            right: None,
        });
        Self::push_node(new_node, &mut self.root);
    }

    // Private and recursive method
    // recursively search through every node until the value is inserted
    fn push_node(new_node: Box<Node<T>>, current_node: &mut Option<Box<Node<T>>>) {
        if let Some(node) = current_node {
            use std::cmp::Ordering;
            match node.val.cmp(&new_node.val) {
                Ordering::Less | Ordering::Equal => Self::push_node(new_node, &mut node.left),
                Ordering::Greater => Self::push_node(new_node, &mut node.right),
            }
        } else {
            current_node.insert(new_node);
        }
    }
}