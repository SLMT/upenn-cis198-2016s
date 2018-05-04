use std::mem;

#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl BST {
    pub fn new() -> BST {
        BST { root: Link::Empty }
    }

    /// Insert an element into the BST. Return true
    /// if successful, or false if the element was already in the BST.
    pub fn insert(&mut self, elem: i32) -> bool {
        self.root.insert(elem)
    }

    /// Search for an element in the BST. Return true
    /// if the element was found.
    pub fn search(&self, elem: i32) -> bool {
        self.root.search(elem)
    }
}

impl Link {
    pub fn insert(&mut self, elem: i32) -> bool {
        match *self {
            Link::Empty => {
                let node = Node {
                    elem: elem,
                    left: Link::Empty,
                    right: Link::Empty
                };
                mem::replace(self, Link::More(Box::new(node)));
                return true;
            },
            Link::More(ref mut boxed_node) => {
                let node = boxed_node.as_mut();
                if elem < node.elem {
                    node.left.insert(elem)
                } else if elem > node.elem {
                    node.right.insert(elem)
                } else {
                    false
                }
            }
        }
    }

    pub fn search(&self, elem: i32) -> bool {
        match *self {
            Link::Empty => false,
            Link::More(ref boxed_node) => {
                let node = boxed_node.as_ref();
                if elem < node.elem {
                    node.left.search(elem)
                } else if elem > node.elem {
                    node.right.search(elem)
                } else {
                    true
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
        let mut tree = BST::new();

        // Check empty list behaves right
        assert_eq!(tree.search(1), false);

        // Populate list
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        // Check search
        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(2), true);
        assert_eq!(tree.search(3), true);
        assert_eq!(tree.search(4), false);

        // Check repeatly insertion
        assert_eq!(tree.insert(1), false);
    }
}