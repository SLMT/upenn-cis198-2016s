
#[derive(Debug)]
pub struct BST<T: PartialOrd> {
    root: Link<T>,
}

#[derive(Debug)]
struct Node<T: PartialOrd> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

trait InsertSearch<T> {
    fn insert(&mut self, elem: T) -> bool;
    fn search(&self, elem: T) -> bool;
}

impl<T: PartialOrd> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    /// Insert an element into the BST. Return true
    /// if successful, or false if the element was already in the BST.
    pub fn insert(&mut self, elem: T) -> bool {
        self.root.insert(elem)
    }

    /// Search for an element in the BST. Return true
    /// if the element was found.
    pub fn search(&self, elem: T) -> bool {
        self.root.search(elem)
    }
}

impl<T: PartialOrd> InsertSearch<T> for Link<T> {
    fn insert(&mut self, elem: T) -> bool {
        match *self {
            None => {
                let node = Node {
                    elem: elem,
                    left: None,
                    right: None
                };
                self.get_or_insert(Box::new(node));
                return true;
            },
            Some(ref mut boxed_node) => {
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

    fn search(&self, elem: T) -> bool {
        match *self {
            None => false,
            Some(ref boxed_node) => {
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

// ============================================== IntoIter

impl<T: PartialOrd> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            stack: vec![],
            tree: self
        }
    }
}

pub struct IntoIter<T: PartialOrd> {
    stack: Vec<Box<Node<T>>>,
    tree: BST<T>
}

impl<T: PartialOrd> IntoIter<T> {
    fn find_next(&mut self, mut node: Box<Node<T>>) -> T {
        if let Some(left_node) = node.left.take() {
            if let Some(right_node) = node.right.take() {
                self.stack.push(right_node);
            }
            self.find_next(left_node)
        } else if let Some(right_node) = node.right.take() {
            self.stack.push(right_node);
            node.elem
        } else {
            node.elem
        }
    }
}

impl<T: PartialOrd> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_start = self.stack.pop();
        if let Some(next_start) = next_start {
            Some(self.find_next(next_start))
        } else {
            if let Some(node) = self.tree.root.take() {
                Some(self.find_next(node))
            } else {
                None
            }
        }
    }
}

// ============================================== Iter

pub struct Iter<'a, T: PartialOrd + 'a> {
    stack: Vec<&'a Node<T>>
}

impl<'a, T: PartialOrd + 'a> Iter<'a, T> {
    fn find_next(&mut self, node: &'a Node<T>) -> &'a T {
        if let Some(ref left_node) = node.left {
            if let Some(ref right_node) = node.right {
                self.stack.push(right_node);
            }
            self.find_next(left_node)
        } else if let Some(ref right_node) = node.right {
            self.stack.push(right_node);
            &node.elem
        } else {
            &node.elem
        }
    }
}

impl<'a, T: PartialOrd + 'a> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        let mut stack = vec![];
        if let Some(ref node) = self.root {
            stack.push(node.as_ref());
        }
        Iter {
            stack: stack
        }
    }
}

impl<'a, T: PartialOrd + 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_start = self.stack.pop();
        if let Some(next_start) = next_start {
            Some(self.find_next(next_start))
        } else {
            None
        }
    }
}

// ============================================== IterMut

pub struct IterMut<'a, T: PartialOrd + 'a> {
    stack: Vec<&'a mut Node<T>>
}

impl<'a, T: PartialOrd + 'a> IterMut<'a, T> {
    fn find_next(&mut self, node: &'a mut Node<T>) -> &'a mut T {
        if let Some(ref mut left_node) = node.left {
            if let Some(ref mut right_node) = node.right {
                self.stack.push(right_node);
            }
            self.find_next(left_node)
        } else if let Some(ref mut right_node) = node.right {
            self.stack.push(right_node);
            &mut node.elem
        } else {
            &mut node.elem
        }
    }
}

impl<'a, T: PartialOrd + 'a> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        let mut stack = vec![];
        if let Some(ref mut node) = self.root {
            stack.push(node.as_mut());
        }
        IterMut {
            stack: stack
        }
    }
}

impl<'a, T: PartialOrd + 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_start = self.stack.pop();
        if let Some(next_start) = next_start {
            Some(self.find_next(next_start))
        } else {
            None
        }
    }
}

// ============================================== Test

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

    #[test]
    fn test_into_iter() {
        let mut tree = BST::new();

        // Populate list
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        // Check into_iterator
        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut tree = BST::new();

        // Populate list
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        // Check into_iterator
        {
            let mut iter = (&tree).into_iter();
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), None);
        }

        // Check repeatly insertion
        assert_eq!(tree.insert(1), false);
    }

    #[test]
    fn test_iter_mut() {
        let mut tree = BST::new();

        // Populate list
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        // Check into_iterator
        {
            let mut iter = (&mut tree).into_iter();
            assert_eq!(iter.next(), Some(&mut 1));
            assert_eq!(iter.next(), Some(&mut 2));
            
            let elem = iter.next(); // it should be 3
            assert_eq!(elem, Some(&mut 3));
            
            // change the value to 4
            if let Some(value) = elem {
                *value = 4;
            }
        }

        // Check insertion
        assert_eq!(tree.insert(1), false);
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(4), false);
    }
}