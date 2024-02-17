// Memtable is Red-Black tree data-structure

use std::cmp::Ordering;
use std::ops::Add;
use std::ptr::null_mut;

#[derive(PartialEq)]
pub enum Side {
    Left,
    Right,
    Root,
}

#[derive(PartialEq)]
pub enum Color {
    Black,
    Red,
}

/// TombStone: value to track delete keys
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Status {
    Available,
    Deleted
}

/// Node contains <key,value>,
/// for using the node in Binary search tree,
/// the key must implement Ord trait(provides min,max.. methods)
/// A pointer to the node can be created using `NodePtr::new()` method
#[allow(dead_code)]
pub struct Node<K: Ord + Clone, V: Clone> {
    key: K,
    value: V,
    timestamp: u128,
    status: Status,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
    parent: NodePtr<K, V>,
    side: Side,
    color: Color,
}

/*
*   Abstraction over pointer to the Node
*   **************NodePtr***************
*/
// We will use unsafe rust and a node store the  pointer to the left ,right and  parent node
/// NodePtr is the abstraction over the pointer to the node
#[derive(Debug)]
pub struct NodePtr<K: Ord + Clone, V:Clone>(*mut Node<K, V>);

impl<K: Ord + Clone, V:Clone> NodePtr<K, V> {
    /// It allcoates a new node in the heap
    /// And saves the raw pointer to the node in the Node Pointer
    pub fn new(key: K, value: V, timestamp: u128,status: Status) -> Self {
        let new_node = Node {
            key,
            value,
            timestamp,
            status: status,
            left: NodePtr::null(),
            right: NodePtr::null(),
            parent: NodePtr::null(),
            side: Side::Root,
            color: Color::Black,
        };
        Self(Box::into_raw(Box::new(new_node)))
    }

    /// sets the right child of the node
    pub fn set_right_child(&mut self, node: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).right = node;
        }
    }

    ///sets the left child of the node
    fn set_left_child(&mut self, node: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).left = node;
        }
    }

    /// sets the side to right side for the node,
    /// to figure out which side of the parent this node lies
    fn set_side_right(&mut self) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).side = Side::Right;
        }
    }

    /// sets the side to left side for the node,
    /// to figure out which side of the parent this node lies
    fn set_side_left(&mut self) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).side = Side::Left;
        }
    }

    /// set the node as root node
    fn set_side_root(&mut self) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).side = Side::Root;
        }
    }

    fn set_color_red(&mut self) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).color = Color::Red;
        }
    }
    fn set_color_black(&mut self) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).color = Color::Black;
        }
    }

    fn set_parent(&mut self, parent: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).parent = parent;
        }
    }
    /// set the node status as deleted
    /// updates timestamp
    /// Cannot change the value to None or some min value
    /// for optimisation, V does't impl default
    fn set_deleted(&mut self, timestamp: u128) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).status = Status::Deleted;
            (*self.0).timestamp = timestamp;
        }
    }

    /// Returns a copy of node's parent
    pub fn get_parent(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).parent.clone() }
    }

    /// returns the value stored inside the node
    // #[allow(unused)]
    pub fn value(&self) -> Option<V> {
        if self.is_null() {
            return None;
        }
        unsafe { Some((*self.0).value.clone()) }
    }
    pub fn set_value(&mut self,v: V) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).value = v }
    }

    fn set_status(&mut self, status: Status) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).status = status;
        }
    }

    #[allow(unused)]
    pub fn key(&self) -> Option<K> {
        if self.is_null() {
            return None;
        }
        unsafe { Some((*self.0).key.clone()) }
    }

    #[allow(unused)]
    pub fn timestamp(&self) -> Option<u128> {
        if self.is_null() {
            return None;
        }
        unsafe { Some((*self.0).timestamp) }
    }

    pub fn set_timestamp(&mut self,timestamp: u128){
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).timestamp = timestamp }
    }

    /// checks if this node's color is red
    pub fn is_red(&self) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { (*self.0).color == Color::Red }
    }

    /// checks if this node's color is black
    pub fn is_black(&self) -> bool {
        if self.is_null() {
            return true;
        }
        unsafe { (*self.0).color == Color::Black }
    }

    /// returns a copy of left child of the node
    pub fn left(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).left.clone() }
    }

    /// returns a copy of right child of the node
    pub fn right(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).right.clone() }
    }

    /// checks if this node locates in the left
    pub fn is_left(&self) -> bool {
        unsafe { (*self.0).side == Side::Left }
    }

    /// checks if this node locates in the right
    pub fn is_right(&self) -> bool {
        unsafe { (*self.0).side == Side::Right }
    }

    /// checks if this key is deleted
    pub fn is_deleted(&self) -> bool {
        unsafe { (*self.0).status == Status::Deleted }
    }

    /// checks if this node is the root node
    pub fn is_root(&self) -> bool {
        unsafe { (*self.0).side == Side::Root }
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn null() -> NodePtr<K, V> {
        NodePtr(null_mut())
    }
}

impl<K: Ord + Clone, V:Clone> Clone for NodePtr<K, V> {
    fn clone(&self) -> NodePtr<K, V> {
        NodePtr(self.0)
    }
}
impl<K: Ord + Clone, V:Clone> Copy for NodePtr<K, V> {}

/// To implement Ord trait one must implement PartialOrd and Eq
/// Implementations must be consistent with the PartialOrd
impl<K: Ord + Clone, V:Clone> Ord for NodePtr<K, V> {
    fn cmp(&self, other: &NodePtr<K, V>) -> Ordering {
        unsafe { (*self.0).key.cmp(&(*other.0).key) }
    }
}

impl<K: Ord + Clone, V:Clone> PartialOrd for NodePtr<K, V> {
    fn partial_cmp(&self, other: &NodePtr<K, V>) -> Option<Ordering> {
        unsafe { Some((*self.0).key.cmp(&(*other.0).key)) }
    }
}
/// To impelement Eq trait, typw must implement PartialEq
impl<K: Ord + Clone, V:Clone> Eq for NodePtr<K, V> {}

impl<K: Ord + Clone, V:Clone> PartialEq for NodePtr<K, V> {
    fn eq(&self, other: &NodePtr<K, V>) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug)]
pub struct RedBlackTree<K: Ord + Clone, V:Clone> {
    pub root: NodePtr<K, V>,
    size: u64,
}

impl<K: Ord + Clone, V:Clone> RedBlackTree<K, V> {
    /// It creates a new Red-Black tree
    pub fn new() -> Self {
        Self {
            root: NodePtr::null(),
            size: 0,
        }
    }
    /// It will insert or replace the node in the binary search tree format
    /// In case of no root, it will be the root node
    pub fn insert_or_replace(
        &mut self, 
        key: K, 
        value: V, 
        timestamp: u128,
        status: Status
    ) {
        // find out if the node is there
        // if not, insert it the tree
        // inserted node is always red
        let mut node = self.find_node(&key);
        if node.is_null() {
            self.insert(key, value,timestamp,status);
            self.size = self.size.add(1);
        } else {
            node.set_value(value);
            node.set_timestamp(timestamp);
            node.set_status(status);
        }
        // TODO: Add else if node is not present
    }
    /// It traverses the tree and return the pointer to the node
    /// if found else return the null Nodeptr
    pub fn find_node(&self, key: &K) -> NodePtr<K, V> {
        if self.root.0.is_null() {
            return NodePtr::null();
        }
        let mut current = &self.root;

        while !current.is_null() {
            unsafe {
                let curr_key = &(*current.0).key;
                let next = match key.cmp(curr_key) {
                    Ordering::Less => &mut (*current.0).left,
                    Ordering::Greater => &mut (*current.0).right,
                    Ordering::Equal => return *current,
                };
                current = next;
            }
        }
        return NodePtr::null();
    }
    /// return true if node with key k is present
    pub fn has_node(&self, k: &K) -> bool {
        let node = self.find_node(k);
        if node.is_null() {
            return false;
        }
        true
    }
    /// Safety: use only if you have checked the node is not present in the tree
    /// It insert the node in the right place
    /// Any inserted node is Red in Color
    fn insert(
        &mut self, 
        key: K, 
        value: V, 
        timestamp: u128,
        status: Status
    ) {
        // if self.root.0.is_null() {
        //     return NodePtr::null()
        // }
        let mut current: NodePtr<K, V> = self.root;
        let mut parent: NodePtr<K, V> = NodePtr::null();

        while !current.is_null() {
    
            unsafe {
                let curr_key = &(*current.0).key;
                let next = match key.cmp(curr_key) {
                    Ordering::Less => (*current.0).left,
                    _ => (*current.0).right, // can never be Equal, as node not present in the tree
                };
                parent = current;
                current = next;
            }
        }

        let mut node = NodePtr::new(key, value,timestamp,status);
        // root node
        if parent.is_null() {
            self.root = node;
            node.set_color_black();     // because root is black
            node.set_side_root();
        } else {
    
            node.set_parent(parent);
    
            node.set_color_red();
    

            match node.cmp(&parent) {
                Ordering::Less => {
            
                    parent.set_left_child(node);
            
                    node.set_side_left();
                }
                _ => {
            
                    parent.set_right_child(node);
            
                    node.set_side_right();
            
                }
            }
        }

        // By now, a node has been set up
        // and now its time to check the Properties of RedBlack tree and make the required change

        self.check_color(node);

        return;
    }

    /// It Recurssively checks for two consecutive red node till the root
    /// In case of violation it fixes the tree then moves up to check further violations
    fn check_color(&mut self, node: NodePtr<K, V>) {
        if node == self.root {
    
            return;
        }
        // it is a violation
        if node.is_red() && node.get_parent().is_red() {
    
            self.correct_tree(node);
    
            self.check_color(node.get_parent());
    
        }
    }

    /// There are two methods of correcting the tree
    /// First is, if the aunt is  black then perform ROTATION
    /// After Rotation, Parent is Black and us and sibbling is red
    /// Second is, if the aunt is Red then we perform the colorflip
    /// After COLORFLIP, GrandParent is Red and parent and aunt is black
    /// GrandParent is aways there, or there is no disbalace is that area
    ///
    /// SAFETY: This is only called from the nodes which is not root
    fn correct_tree(&mut self, node: NodePtr<K, V>) {
        // find aunt's color, null is black so null aunt is black
        // if parent is in left side then aunt will be on right side
        // if parent is in right side then aunt will be on left side
        if node.get_parent().is_left() {
            // aunt is black, rotate
            if node.get_parent().get_parent().right().is_null()
                || node.get_parent().get_parent().right().is_black()
            {
                return self.rotate(node);
            }

            // else aunt is red, do color flip
            node.get_parent().set_color_black();
            if !node.get_parent().get_parent().is_root() {
                node.get_parent().get_parent().set_color_red();             // Only if not root node,
            }

            if !node.get_parent().get_parent().right().is_null() {
                node.get_parent().get_parent().right().set_color_black()
            }
        } else {
            // aunt is black, rotate
            if node.get_parent().get_parent().left().is_null()
                || node.get_parent().get_parent().left().is_black()
            {
                return self.rotate(node);
            }

            // else aunt is red, do color flip
            if !node.get_parent().get_parent().is_root() {
                node.get_parent().get_parent().set_color_black();
            }
            node.get_parent().set_color_black();             // Only if not root node,

            if !node.get_parent().get_parent().left().is_null() {
                node.get_parent().get_parent().left().set_color_black()
            }
        }
    }

    fn rotate(&mut self, mut node: NodePtr<K, V>) {
        if node.is_left() {
            if node.get_parent().is_left() {
                // perform right rotate, pass grandparent
                self.right_rotate(node.get_parent().get_parent());
                // set color after rotation
                node.set_color_red();
                node.get_parent().set_color_black();
                if !node.get_parent().right().is_null() {
                    node.get_parent().right().set_color_red();
                }
                return;
            }
            self.right_left_rotate(node.get_parent().get_parent());
            // after right left rotation, the node we start from become parent
            // and GP and parent become the child, so coloring

            node.set_color_black();     // because it is parent now
            node.left().set_color_red();    
            node.right().set_color_red();
            return;
        } else {
            if node.get_parent().is_right() {
                // perform left rotate, pass grandparent
                self.left_rotate(node.get_parent().get_parent());
                // set color after rotation
                node.set_color_red();
                node.get_parent().set_color_black();
                if !node.get_parent().left().is_null() {
                    node.get_parent().left().set_color_red();
                }
                return;
            }
            self.left_right_rotate(node.get_parent().get_parent());
            // after left right rotation, the node we start from become parent
            // and GP and parent become the child, so coloring

            node.set_color_black();
            node.left().set_color_red();
            node.right().set_color_red();
            return;
        }
    }

    fn left_rotate(&mut self, mut node: NodePtr<K, V>) {
        let mut temp = node.right();
        node.set_right_child(temp.left());

        // parent node's left maybe null
        if !node.right().is_null() {
            node.right().set_parent(node);
            node.right().set_side_right();
        }
        // node was root node, assign root to parent
        if node.is_root() {
            self.root = temp;
            temp.set_side_root();

        } else {
            // set the parents node's left and right node accordingly
            if node.is_left() {
                node.get_parent().set_left_child(temp);
                temp.set_side_left();
            } else {
                node.set_right_child(temp);
                temp.set_side_right();
            }
        }
        temp.set_parent(node.get_parent());
        temp.set_left_child(node);
        node.set_parent(temp);
        node.set_side_left(); 
    }
    
    fn right_rotate(&mut self, mut node: NodePtr<K, V>) {
        let mut temp = node.left();
        node.set_left_child(temp.right());
    
        // parent node's left maybe null
        if !node.left().is_null() {
            node.left().set_parent(node);
            node.left().set_side_left();
        }
        // node was root node, assign root to parent
        if node.is_root() {
            self.root = temp;
            temp.set_side_root();
        } else {
            // set the parents node's left and right node accordingly
            if node.is_left() {
                node.get_parent().set_left_child(temp);
                temp.set_side_left();
            } else {
                node.set_right_child(temp);
                temp.set_side_right();
            }
        }
        temp.set_parent(node.get_parent());
        temp.set_right_child(node);
        node.set_parent(temp);
        node.set_side_right(); 

    }

    // node is Grand parent,so first rotate left by taking hinge at parent
    // then rotate right by taking hinge by grand parent
    fn left_right_rotate(&mut self, node: NodePtr<K, V>) {
        self.left_rotate(node.left());
        self.right_rotate(node);
    }
    fn right_left_rotate(&mut self, node: NodePtr<K, V>) {
        self.right_rotate(node.right());
        self.left_rotate(node);
    }

    pub fn check_key_deleted(
        &self,
        key: &K
    ) -> bool {
        let node = self.find_node(key);
        if node.is_deleted() {
            return true;
        }
        return false;
    }

    pub fn delete_key(
        &self,
        key: &K,
        timestamp: u128
    ) {
        let mut node = self.find_node(key);
        node.set_deleted(timestamp);
    }

    pub fn value(
        &self,
        key: &K
    ) -> Option<V> {
        let node = self.find_node(key);
        
        return node.value();
    }
}


#[cfg(test)]
mod tests {

    use std::ptr::null_mut;

    use crate::engine::mem_table::red_black_tree::red_black_tree::NodePtr;

    use super::*;

    #[test]
    fn insert_nothing() {
        let rb = RedBlackTree::<u8,u8>::new();
        assert_eq!(rb.size,0);
        assert_eq!(rb.root,NodePtr(null_mut()))
    }

    #[test]
    fn insert_root() {
        let mut rb = RedBlackTree::<u8,u8>::new();
        rb.insert_or_replace(11, 16, 0, Status::Available);

        // check tree status
        assert_eq!(rb.size,1);
        assert_ne!(rb.root,NodePtr(null_mut()));
        

        // check root node status, all types in node
        let x = rb.find_node(&11);
        assert_eq!(x.value(),Some(16));
        assert_eq!(x.left().is_null(), true);
        assert_eq!(x.right().is_null(), true);
        assert_eq!(x.get_parent().is_null(), true);
        assert_eq!(x.is_root(),true);
        assert_eq!(x.is_black(),true);
    }

    /*
    *     11
    *    /  \
    *   8   14
    */
    #[test]
    fn insert_root_left_right() {
        let mut rb = RedBlackTree::<u8,u8>::new();
        rb.insert_or_replace(11, 16,0, Status::Available);
        rb.insert_or_replace(14, 19,0, Status::Available);
        rb.insert_or_replace(8, 13,0, Status::Available);

        // check tree status
        assert_eq!(rb.size,3);
        assert_ne!(rb.root,NodePtr(null_mut()));
        

        // check root node status, all types in node
        let x = rb.find_node(&11);
        let y = rb.find_node(&8);
        let z = rb.find_node(&14);
        assert_eq!(x.value(),Some(16));
        assert_eq!(x.left(),y);
        assert_eq!(x.right(),z);
        assert_eq!(x.get_parent().is_null(), true);
        assert_eq!(x.is_root(),true);
        assert_eq!(x.is_black(),true);

        // check left node status, all types in node
        assert_eq!(y.value(),Some(13));
        assert_eq!(y.left().is_null(),true);
        assert_eq!(y.right().is_null(),true);
        assert_eq!(y.get_parent(), x);
        assert_eq!(y.is_left(),true);
        assert_eq!(y.is_red(),true);

        // check right node status, all types in node
        assert_eq!(z.value(),Some(19));
        assert_eq!(z.left().is_null(),true);
        assert_eq!(z.right().is_null(),true);
        assert_eq!(z.get_parent(), x);
        assert_eq!(z.is_right(),true);
        assert_eq!(z.is_red(),true);
    }


    /*
    *           11 B
    *          /  \
    *       R 8   14 R
    *        /
    *     R 7 -> new insert,Red aunt 
    *           POST
    *           11 B
    *          /  \
    *       B 8   14 B
    *        /
    *     R 7 -> new insert,Red aunt 
    */
    #[test]
    fn check_color_flip() {
        let mut rb = RedBlackTree::<u8,u8>::new();
        rb.insert_or_replace(11, 16, 0, Status::Available);
        rb.insert_or_replace(14, 19, 0, Status::Available);
        rb.insert_or_replace(8, 13, 0, Status::Available);
        rb.insert_or_replace(7, 12, 0, Status::Available);

        // check tree status
        assert_eq!(rb.size,4);
        assert_ne!(rb.root,NodePtr(null_mut()));
        

        // check root node status
        let x = rb.find_node(&11);
        let y = rb.find_node(&8);
        let z = rb.find_node(&14);
        let w = rb.find_node(&7);
        
        assert_eq!(x.value(),Some(16));
        assert_eq!(x.left(),y);
        assert_eq!(x.right(),z);
        assert_eq!(x.get_parent().is_null(), true);
        assert_eq!(x.is_root(),true);
        assert_eq!(x.is_black(),true);

        // check left node status
        assert_eq!(y.value(),Some(13));
        assert_eq!(y.left(),w);
        assert_eq!(y.right().is_null(),true);
        assert_eq!(y.get_parent(), x);
        assert_eq!(y.is_left(),true);
        assert_eq!(y.is_black(),true);

        // check right node status
        assert_eq!(z.value(),Some(19));
        assert_eq!(z.left().is_null(),true);
        assert_eq!(z.right().is_null(),true);
        assert_eq!(z.get_parent(), x);
        assert_eq!(z.is_right(),true);
        assert_eq!(z.is_black(),true);

        // check new inserted node status
        assert_eq!(w.value(),Some(12));
        assert_eq!(w.left().is_null(),true);
        assert_eq!(w.right().is_null(),true);
        assert_eq!(w.get_parent(), y);
        assert_eq!(w.is_left(),true);
        assert_eq!(w.is_red(),true);
    }
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

        // check tree status
        assert_eq!(rb.size,5);
        assert_ne!(rb.root,NodePtr(null_mut()));
        

        // check root node status
        let root = rb.find_node(&11);
        let l = rb.find_node(&7);
        let r = rb.find_node(&14);
        let ll = rb.find_node(&6);
        let lr = rb.find_node(&8);
        
        assert_eq!(rb.has_node(&11),true);
        assert_eq!(rb.has_node(&7),true);
        assert_eq!(rb.has_node(&14),true);
        assert_eq!(rb.has_node(&6),true);
        assert_eq!(rb.has_node(&8),true);
        
        assert_eq!(root.value(),Some(16));
        assert_eq!(root.left(),l);
        assert_eq!(root.right(),r);
        assert_eq!(root.get_parent().is_null(), true);
        assert_eq!(root.is_root(),true);
        assert_eq!(root.is_black(),true);

        // check left node status
        assert_eq!(l.value(),Some(12));
        assert_eq!(l.left(),ll);
        assert_eq!(l.right(),lr);
        assert_eq!(l.get_parent(), root);
        assert_eq!(l.is_left(),true);
        assert_eq!(l.is_black(),true);

        // // check right node status
        assert_eq!(r.value(),Some(19));
        assert_eq!(r.left().is_null(),true);
        assert_eq!(r.right().is_null(),true);
        assert_eq!(r.get_parent(), root);
        assert_eq!(r.is_right(),true);
        assert_eq!(r.is_black(),true);

        // // check new inserted node status
        assert_eq!(ll.value(),Some(11));
        assert_eq!(ll.left().is_null(),true);
        assert_eq!(ll.right().is_null(),true);
        assert_eq!(ll.get_parent(), l);
        assert_eq!(ll.is_left(),true);
        assert_eq!(ll.is_red(),true);

        // check parent which get pulled in the right side
        assert_eq!(lr.value(),Some(13));
        assert_eq!(lr.left().is_null(),true);
        assert_eq!(lr.right().is_null(),true);
        assert_eq!(lr.get_parent(), l);
        assert_eq!(lr.is_right(),true);
        assert_eq!(lr.is_red(),true);
    }

    #[test]
    fn increasing_insertions(){
        let mut rb = RedBlackTree::<u8,u8>::new();
        for i in 1..60 {
            rb.insert_or_replace(i, i, 0, Status::Available);
        }

        let mut iter = rb.root.into_iter();
        for i in 1..60 {
            assert_eq!(Some(i),iter.next().unwrap().value());
        }
        assert_eq!(None,iter.next());
    }
    #[test]
    fn decreasing_insertions(){
        let mut rb = RedBlackTree::<u8,u8>::new();
        for i in (1..60).rev(){
            rb.insert_or_replace(i, i, 0, Status::Available);
        }

        let mut iter = rb.root.into_iter();
        for i in 1..60 {
            assert_eq!(Some(i),iter.next().unwrap().value());
        }
        assert_eq!(None,iter.next());
    }

    #[test]
    fn reset_same_key(){
        let mut rb = RedBlackTree::<u8,u8>::new();
        for i in 1..60 {
    
            rb.insert_or_replace(i, i, 0, Status::Available);
        }
        
        let mut iter = rb.root.into_iter();
        for i in 1..60 {
            rb.insert_or_replace(i, i+5, 0, Status::Available);
        }

        for i in 1..60 {
            assert_eq!(Some(i+5),iter.next().unwrap().value());
        }
        assert_eq!(None,iter.next());
    }
}
