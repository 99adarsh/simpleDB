use std::collections::VecDeque;

use super::red_black_tree::{NodePtr, RedBlackTree};

/*
* Inorder Iterator can be implemented using stack but takes O(n) space,
* The implementation is Morris traversal(https://www.geeksforgeeks.org/morris-traversal-for-preorder/) which is done in O(1) space
*/

/// Inorder iterator, yield next item in an inorder tree traversal fashion
/// By using Morris Traversal
pub struct InorderIterator<K: Ord, V: Clone> {
    current: NodePtr<K, V>,
}

impl<K:Ord,V:Clone> InorderIterator<K,V> {
    pub fn new(root: NodePtr<K,V>) -> Self {
        Self {
            current: root
        }
    }
}

impl<K: Ord, V: Clone> Iterator for InorderIterator<K, V> {
    type Item = NodePtr<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        if self.current.left().is_null() {
            let temp = Some(self.current);
            self.current = self.current.right();
            return temp;
        } else {
            let mut right_most = self.current.left();

            while !right_most.right().is_null() 
                && right_most.right() != self.current {
                right_most = right_most.right();
            }

            if right_most.right().is_null() {
                right_most.set_right_child(self.current);
                self.current = self.current.left();
            } else {
                right_most.set_right_child(NodePtr::null());
                let temp = self.current;
                self.current = self.current.right();
                return Some(temp);
            }
        }
        return self.next();
    }
}

impl<K:Ord,V:Clone> IntoIterator for NodePtr<K,V> {
    type Item = NodePtr<K, V>;

    type IntoIter = InorderIterator<K,V>;

    fn into_iter(self) -> Self::IntoIter {
        InorderIterator::new(self)
    }
}

