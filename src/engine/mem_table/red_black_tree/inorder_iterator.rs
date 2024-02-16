use super::red_black_tree::{NodePtr, RedBlackTree, Status};

/*
* InOrd + Cloneer Iterator can be implemented using stack but takes O(n) space,
* The below implementation is Morris traversal(https://www.geeksforgeeks.org/morris-traversal-for-preOrd + Cloneer/) 
* which takes O(1) space only
*/

/// InOrd + Cloneer iterator, yield next item in an inOrd + Cloneer tree traversal fashion
/// By using Morris Traversal
pub struct InOrderIterator<K: Ord + Clone, V: Clone> {
    current: NodePtr<K, V>,
}

impl<K:Ord + Clone,V:Clone> InOrderIterator<K,V> {
    pub fn new(root: NodePtr<K,V>) -> Self {
        Self {
            current: root
        }
    }
}

impl<K: Ord + Clone, V: Clone> Iterator for InOrderIterator<K, V> {
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

impl<K:Ord + Clone,V:Clone> IntoIterator for NodePtr<K,V> {
    type Item = NodePtr<K, V>;

    type IntoIter = InOrderIterator<K,V>;

    fn into_iter(self) -> Self::IntoIter {
        InOrderIterator::new(self)
    }
}

# [cfg(test)]
mod tests {
    use super::*;
    /*
    *           11 B
    *          /  \
    *       B 8   14 B
    *        /
    *     R 7 
    *      /
    *   R 6 -> new insert
    *       POST
    *           11 B
    *          /  \
    *       B 7   14 B
    *        / \
    *     R 6   8 R
    */
        
    #[test]
    fn check_right_rotate() {
        let mut rb = RedBlackTree::<u8,u8>::new();
        rb.insert_or_replace(11, 16, 0, Status::Available);
        rb.insert_or_replace(14, 19, 0, Status::Available);
        rb.insert_or_replace(8, 13, 0, Status::Available);
        rb.insert_or_replace(7, 12, 0, Status::Available);
        rb.insert_or_replace(6, 11, 0, Status::Available);

        let mut iter = rb.root.into_iter();
        // let next = iter.next();
        assert_eq!(iter.next().unwrap().value(),Some(11));
        assert_eq!(iter.next().unwrap().value(),Some(12));
        assert_eq!(iter.next().unwrap().value(),Some(13));
        assert_eq!(iter.next().unwrap().value(),Some(16));
        assert_eq!(iter.next().unwrap().value(),Some(19));
        assert_eq!(iter.next(),None);
    }
}