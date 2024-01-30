// Memtable is Red-Black tree data-structure

use std::cmp::Ordering;
use std::ops::Add;
use std::ptr::null_mut;

#[derive(PartialEq)]
enum Side {
    Left,
    Right,
    Root,
}

#[derive(PartialEq)]
enum Color {
    Black,
    Red,
}
/// Node contains <key,value>,
/// for using the node in Binary search tree,
/// the key must implement Ord trait(provides min,max.. methods)
/// A pointer to the node can be created using `NodePtr::new()` method
#[allow(dead_code)]
pub struct Node<K: Ord, V> {
    key: K,
    value: V,
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
// #[derive(Copy)]
struct NodePtr<K: Ord, V>(*mut Node<K, V>);

impl<K: Ord, V> NodePtr<K, V> {
    /// It allcoates a new node in the heap
    /// And saves the raw pointer to the node in the Node Pointer
    pub fn new(key: K, value: V) -> Self {
        let new_node = Node {
            key,
            value,
            left: NodePtr::null(),
            right: NodePtr::null(),
            parent: NodePtr::null(),
            side: Side::Root,
            color: Color::Black,
        };
        Self(Box::into_raw(Box::new(new_node)))
    }

    /// sets the right child of the node
    fn set_right_child(&mut self, node: NodePtr<K, V>) {
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

    /// Returns a copy of node's parent
    fn get_parent(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).parent.clone() }
    }

    /// checks if this node's color is red
    fn is_red(&self) -> bool {
        unsafe { (*self.0).color == Color::Red }
    }

    /// checks if this node's color is black
    fn is_black(&self) -> bool {
        unsafe { (*self.0).color == Color::Black }
    }

    /// returns a copy of left child of the node
    fn left(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).left.clone() }
    }

    /// returns a copy of right child of the node
    fn right(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).right.clone() }
    }

    /// checks if this node locates in the left
    fn is_left(&self) -> bool {
        unsafe { (*self.0).side == Side::Left }
    }

    /// checks if this node locates in the right
    fn is_right(&self) -> bool {
        unsafe { (*self.0).side == Side::Right }
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }

    fn null() -> NodePtr<K, V> {
        NodePtr(null_mut())
    }
}

impl<K: Ord, V> Clone for NodePtr<K, V> {
    fn clone(&self) -> NodePtr<K, V> {
        NodePtr(self.0)
    }
}
impl<K: Ord, V> Copy for NodePtr<K, V> {}

/// To implement Ord trait one must implement PartialOrd and Eq
/// Implementations must be consistent with the PartialOrd
impl<K: Ord, V> Ord for NodePtr<K, V> {
    fn cmp(&self, other: &NodePtr<K, V>) -> Ordering {
        unsafe { (*self.0).key.cmp(&(*other.0).key) }
    }
}

impl<K: Ord, V> PartialOrd for NodePtr<K, V> {
    fn partial_cmp(&self, other: &NodePtr<K, V>) -> Option<Ordering> {
        unsafe { Some((*self.0).key.cmp(&(*other.0).key)) }
    }
}
/// To impelement Eq trait, typw must implement PartialEq
impl<K: Ord, V> Eq for NodePtr<K, V> {}

impl<K: Ord, V> PartialEq for NodePtr<K, V> {
    fn eq(&self, other: &NodePtr<K, V>) -> bool {
        self.0 == other.0
    }
}

pub struct RedBlackTree<K: Ord, V> {
    root: NodePtr<K, V>,
    size: u64,
}

impl<K: Ord, V> RedBlackTree<K, V> {
    /// It creates a new Red-Black tree
    pub fn new() -> Self {
        Self {
            root: NodePtr::null(),
            size: 0,
        }
    }
    /// It will insert or replace the node in the binary search tree format
    /// In case of no root, it will be the root node
    pub fn insert_or_replace(&mut self, key: K, value: V) {
        // find out if the node is there
        // if not, insert it the tree
        // inserted node is always red
        let node = self.find_node(&key);

        if node.is_null() {
            self.insert(key, value);
            self.size = self.size.add(1);
        }
    }
    /// It traverses the tree and return the pointer to the node
    /// if found else return the null Nodeptr
    fn find_node(&self, key: &K) -> NodePtr<K, V> {
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
    /// Safety: use only if you have checked the node is not present in the tree
    /// It insert the node in the right place
    /// Any inserted node is Red in Color
    fn insert(&mut self, key: K, value: V) {
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
        let mut node = NodePtr::new(key, value);
        // root node
        if parent.is_null() {
            self.root = node;
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
    /// After Rotation, GrandParent is Black and parent and aunt is red
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
            node.get_parent().get_parent().set_color_red();

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
            node.get_parent().set_color_black();
            node.get_parent().get_parent().set_color_red();

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
        if node.get_parent().is_null() {
            self.root = temp;
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
        if node.get_parent().is_null() {
            self.root = temp;
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
}
