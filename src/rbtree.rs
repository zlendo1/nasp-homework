use std::{borrow::Borrow, cmp::Ordering, fmt::Debug, ptr::NonNull};

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    Red,
}

#[derive(Debug)]
struct Node<K> {
    parent: Link<K>,
    left: Link<K>,
    right: Link<K>,
    color: Color,
    key: K,
}

type NodePtr<K> = NonNull<Node<K>>;
type Link<K> = Option<NodePtr<K>>;
type LinkPtr<K> = NonNull<Link<K>>;

enum InsertPos<K> {
    Vacant {
        parent: Link<K>,
        link_ptr: LinkPtr<K>,
    },
    Occupied {},
}

impl<K: Debug> Node<K> {
    fn create(parent: Link<K>, key: K) -> NodePtr<K> {
        let boxed = Box::new(Node {
            parent,
            left: None,
            right: None,
            color: Color::Red,
            key,
        });

        NodePtr::from(Box::leak(boxed))
    }

    unsafe fn destroy(node_ptr: NodePtr<K>) -> K {
        let boxed = Box::from_raw(node_ptr.as_ptr());

        boxed.key
    }

    fn is_red(&self) -> bool {
        match self.color {
            Color::Black => false,
            Color::Red => true,
        }
    }

    fn is_black(&self) -> bool {
        return !self.is_red();
    }

    fn has_child(&self) -> bool {
        return self.left.is_some() || self.right.is_some();
    }

    fn has_left(&self) -> bool {
        return self.left.is_some();
    }

    fn has_right(&self) -> bool {
        return self.right.is_some();
    }
}

#[derive(Debug)]
pub struct RbTree<K> {
    root: Link<K>,
}

#[allow(dead_code)]
impl<K> RbTree<K>
where
    K: Copy + Debug,
{
    pub fn new() -> Self
    where
        K: Ord,
    {
        Self { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn find<Q>(&self, key: &Q) -> Link<K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self.root;
        while let Some(node_ptr) = current {
            current = unsafe {
                match key.cmp(node_ptr.as_ref().key.borrow()) {
                    Ordering::Equal => break,
                    Ordering::Less => node_ptr.as_ref().left,
                    Ordering::Greater => node_ptr.as_ref().right,
                }
            }
        }
        current
    }

    pub fn clear(&mut self) {
        self.recursive_destroy(self.root);

        self.root = None;
    }

    fn recursive_destroy(&mut self, node: Link<K>) {
        match node {
            None => return,
            Some(mut node_ptr) => unsafe {
                self.recursive_destroy(node_ptr.as_mut().left);
                self.recursive_destroy(node_ptr.as_mut().right);
                Node::destroy(node_ptr);
            },
        }
    }

    pub fn inorder_traverse(&self) -> Vec<K> {
        let mut result: Vec<K> = Vec::new();

        self.inorder(self.root.borrow(), &mut result);

        result
    }

    fn inorder(&self, node: &Link<K>, result: &mut Vec<K>) {
        match node {
            None => return,
            Some(node_ptr) => unsafe {
                let key = node_ptr.as_ref().key;

                self.inorder(node_ptr.as_ref().left.borrow(), result);
                result.push(key);
                self.inorder(node_ptr.as_ref().right.borrow(), result);
            },
        }
    }

    pub fn preorder_tranverse(&self) -> Vec<K> {
        let mut result: Vec<K> = Vec::new();

        self.preorder(self.root.borrow(), &mut result);

        result
    }

    fn preorder(&self, node: &Link<K>, result: &mut Vec<K>) {
        match node {
            None => return,
            Some(node_ptr) => unsafe {
                let key = node_ptr.as_ref().key;

                result.push(key);
                self.preorder(node_ptr.as_ref().left.borrow(), result);
                self.preorder(node_ptr.as_ref().right.borrow(), result);
            },
        }
    }

    fn rotate_left(&mut self, mut node_ptr: NodePtr<K>) {
        unsafe {
            if let Some(mut right_ptr) = node_ptr.as_ref().right {
                node_ptr.as_mut().right = right_ptr.as_ref().left;
                if let Some(mut right_left_ptr) = right_ptr.as_mut().left {
                    right_left_ptr.as_mut().parent = Some(node_ptr);
                }

                right_ptr.as_mut().parent = node_ptr.as_ref().parent;
                match node_ptr.as_ref().parent {
                    None => self.root = Some(right_ptr),
                    Some(mut parent_ptr) => {
                        if parent_ptr.as_ref().left == Some(node_ptr) {
                            parent_ptr.as_mut().left = Some(right_ptr);
                        } else {
                            parent_ptr.as_mut().right = Some(right_ptr);
                        }
                    }
                }

                right_ptr.as_mut().left = Some(node_ptr);
                node_ptr.as_mut().parent = Some(right_ptr);
            }
        }
    }

    fn rotate_right(&mut self, mut node_ptr: NodePtr<K>) {
        unsafe {
            if let Some(mut left_ptr) = node_ptr.as_ref().left {
                node_ptr.as_mut().left = left_ptr.as_ref().right;
                if let Some(mut right_ptr) = left_ptr.as_ref().right {
                    right_ptr.as_mut().parent = Some(node_ptr);
                }

                left_ptr.as_mut().parent = node_ptr.as_ref().parent;
                match node_ptr.as_ref().parent {
                    None => self.root = Some(left_ptr),
                    Some(mut parent_ptr) => {
                        if parent_ptr.as_ref().left == Some(node_ptr) {
                            parent_ptr.as_mut().left = Some(left_ptr);
                        } else {
                            parent_ptr.as_mut().right = Some(left_ptr);
                        }
                    }
                }

                left_ptr.as_mut().right = Some(node_ptr);
                node_ptr.as_mut().parent = Some(left_ptr);
            }
        }
    }

    pub fn insert(&mut self, key: K) -> bool
    where
        K: Ord,
    {
        match self.find_insert_pos(&key) {
            InsertPos::Vacant { parent, link_ptr } => unsafe {
                self.insert_entry_at_pos(parent, link_ptr, key);
                true
            },
            InsertPos::Occupied {} => false,
        }
    }

    fn find_insert_pos<Q>(&mut self, key: &Q) -> InsertPos<K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut parent: Link<K> = None;
        let mut link_ptr: LinkPtr<K> = unsafe { LinkPtr::new_unchecked(&mut self.root) };

        unsafe {
            while let Some(mut node_ptr) = link_ptr.as_ref() {
                if key == node_ptr.as_ref().key.borrow() {
                    return InsertPos::Occupied {};
                } else {
                    parent = *link_ptr.as_ref();
                    if key < node_ptr.as_ref().key.borrow() {
                        link_ptr = LinkPtr::new_unchecked(&mut node_ptr.as_mut().left);
                    } else {
                        link_ptr = LinkPtr::new_unchecked(&mut node_ptr.as_mut().right);
                    }
                }
            }
        }

        InsertPos::Vacant { parent, link_ptr }
    }

    unsafe fn insert_entry_at_pos(&mut self, parent: Link<K>, mut insert_pos: LinkPtr<K>, key: K) {
        let node_ptr = Node::create(parent, key);

        *insert_pos.as_mut() = Some(node_ptr);

        self.balance_insert(node_ptr);
    }

    fn balance_insert(&mut self, start_from: NodePtr<K>) {
        let mut current = Some(start_from);

        unsafe {
            while let Some(mut node_ptr) = current {
                let parent = node_ptr.as_mut().parent;
                if parent.is_none() || parent.unwrap().as_ref().is_black() {
                    break;
                }

                let mut parent_ptr = parent.unwrap();
                let grandparent = parent_ptr.as_mut().parent;
                if grandparent.is_none() {
                    break;
                }

                let mut grandparent_ptr = grandparent.unwrap();
                let left_uncle = grandparent_ptr.as_mut().left;
                let right_uncle = grandparent_ptr.as_mut().right;
                let is_left = parent == left_uncle;

                let uncle = if is_left { right_uncle } else { left_uncle };
                if let Some(mut uncle_ptr) = uncle {
                    if uncle_ptr.as_ref().is_red() {
                        parent_ptr.as_mut().color = Color::Black;
                        uncle_ptr.as_mut().color = Color::Black;
                        grandparent_ptr.as_mut().color = Color::Red;
                        current = grandparent;
                        continue;
                    }
                }

                if is_left {
                    if current == parent_ptr.as_ref().right {
                        current = Some(parent_ptr);
                        node_ptr = current.unwrap();
                        self.rotate_left(node_ptr);
                        parent_ptr = node_ptr.as_mut().parent.unwrap();
                    }
                    parent_ptr.as_mut().color = Color::Black;
                    grandparent_ptr.as_mut().color = Color::Red;
                    self.rotate_right(grandparent_ptr);
                } else {
                    if current == parent_ptr.as_ref().left {
                        current = Some(parent_ptr);
                        node_ptr = current.unwrap();
                        self.rotate_right(node_ptr);
                        parent_ptr = node_ptr.as_mut().parent.unwrap();
                    }
                    parent_ptr.as_mut().color = Color::Black;
                    grandparent_ptr.as_mut().color = Color::Red;
                    self.rotate_left(grandparent_ptr);
                }
            }

            if let Some(mut root_ptr) = self.root {
                root_ptr.as_mut().color = Color::Black;
            }
        }
    }

    pub fn delete<Q>(&mut self, key: &Q) -> Option<K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let found = self.find(key);

        if found.is_none() {
            return None;
        }

        let node_ptr = found.unwrap();
        let key = unsafe { self.remove_entry_at_occupied_pos(node_ptr) };

        Some(key)
    }

    unsafe fn remove_entry_at_occupied_pos(&mut self, mut node_ptr: NodePtr<K>) -> K {
        debug_assert!(!self.is_empty());

        let mut is_min_child_black = node_ptr.as_ref().is_black();
        let replacement: Option<NonNull<Node<K>>>;

        let mut leaf_deletion = false;

        if !node_ptr.as_ref().has_child() {
            replacement = Some(node_ptr);
            leaf_deletion = true;
        } else if !node_ptr.as_ref().has_left() {
            replacement = node_ptr.as_mut().right;
            self.transplant(node_ptr, node_ptr.as_mut().right);
        } else if !node_ptr.as_ref().has_right() {
            replacement = node_ptr.as_mut().left;
            self.transplant(node_ptr, node_ptr.as_mut().left);
        } else {
            let mut min_child_ptr = self.minimum(node_ptr.as_ref().right.unwrap());
            is_min_child_black = min_child_ptr.as_ref().is_black();
            replacement = min_child_ptr.as_mut().right;

            if Some(min_child_ptr) != node_ptr.as_mut().right {
                self.transplant(min_child_ptr, min_child_ptr.as_mut().right);

                min_child_ptr.as_mut().right = node_ptr.as_ref().right;
                min_child_ptr.as_mut().right.unwrap().as_mut().parent = Some(min_child_ptr);
            } else if let Some(mut replacement_ptr) = replacement {
                replacement_ptr.as_mut().parent = Some(min_child_ptr);
            }

            self.transplant(node_ptr, Some(min_child_ptr));
            min_child_ptr.as_mut().left = node_ptr.as_ref().left;
            min_child_ptr.as_mut().left.unwrap().as_mut().parent = Some(min_child_ptr);
            min_child_ptr.as_mut().color = node_ptr.as_ref().color;
        }

        if is_min_child_black {
            self.balance_delete(replacement);
        }

        if leaf_deletion {
            self.transplant(node_ptr, None);
        }

        Node::destroy(node_ptr)
    }

    fn minimum(&self, node_ptr: NodePtr<K>) -> NodePtr<K> {
        unsafe {
            if !node_ptr.as_ref().has_left() {
                return node_ptr;
            }

            self.minimum(node_ptr.as_ref().left.unwrap())
        }
    }

    unsafe fn transplant(&mut self, mut node_ptr: NodePtr<K>, replacement: Link<K>) {
        if node_ptr.as_ref().parent.is_none() {
            self.root = replacement;
        } else if Some(node_ptr) == node_ptr.as_mut().parent.unwrap().as_ref().left {
            node_ptr.as_mut().parent.unwrap().as_mut().left = replacement;
        } else {
            node_ptr.as_mut().parent.unwrap().as_mut().right = replacement;
        }

        if let Some(mut replacement_ptr) = replacement {
            replacement_ptr.as_mut().parent = node_ptr.as_mut().parent;
        }
    }

    unsafe fn balance_delete(&mut self, mut link: Link<K>) {
        debug_assert!(link.is_some());

        while link != self.root && link.unwrap().as_ref().is_black() {
            let mut node_ptr = link.unwrap();

            macro_rules! parent_ptr {
                () => {
                    node_ptr.as_mut().parent.unwrap()
                };
            }
            macro_rules! left_sibling {
                () => {
                    parent_ptr!().as_mut().left
                };
            }
            macro_rules! right_sibling {
                () => {
                    parent_ptr!().as_mut().right
                };
            }

            let is_left_child: bool = link == left_sibling!();

            println!(
                "{} {}",
                right_sibling!().is_some(),
                left_sibling!().is_some()
            );

            let mut sibling_ptr = if is_left_child {
                right_sibling!().unwrap()
            } else {
                left_sibling!().unwrap()
            };

            if sibling_ptr.as_ref().is_red() {
                sibling_ptr.as_mut().color = Color::Black;
                parent_ptr!().as_mut().color = Color::Red;

                if is_left_child {
                    self.rotate_left(parent_ptr!());
                } else {
                    self.rotate_right(parent_ptr!());
                }

                sibling_ptr = if is_left_child {
                    right_sibling!().unwrap()
                } else {
                    left_sibling!().unwrap()
                }
            }

            macro_rules! primary_nephew {
                () => {
                    if is_left_child {
                        sibling_ptr.as_mut().left
                    } else {
                        sibling_ptr.as_mut().right
                    }
                };
            }
            macro_rules! secondary_nephew {
                () => {
                    if is_left_child {
                        sibling_ptr.as_mut().right
                    } else {
                        sibling_ptr.as_mut().left
                    }
                };
            }

            if (primary_nephew!().is_none() || primary_nephew!().unwrap().as_mut().is_black())
                && (secondary_nephew!().is_none()
                    || secondary_nephew!().unwrap().as_mut().is_black())
            {
                sibling_ptr.as_mut().color = Color::Red;
                link = node_ptr.as_mut().parent;
                continue;
            }

            if secondary_nephew!().is_none() || secondary_nephew!().unwrap().as_mut().is_black() {
                if let Some(mut primary_nephew_ptr) = primary_nephew!() {
                    primary_nephew_ptr.as_mut().color = Color::Black;
                }
                sibling_ptr.as_mut().color = Color::Red;

                if is_left_child {
                    self.rotate_right(sibling_ptr);
                } else {
                    self.rotate_left(sibling_ptr);
                }

                sibling_ptr = if is_left_child {
                    right_sibling!().unwrap()
                } else {
                    left_sibling!().unwrap()
                }
            }

            sibling_ptr.as_mut().color = parent_ptr!().as_ref().color;
            parent_ptr!().as_mut().color = Color::Black;
            if let Some(mut secondary_nephew_ptr) = secondary_nephew!() {
                secondary_nephew_ptr.as_mut().color = Color::Black;
            }

            if is_left_child {
                self.rotate_left(parent_ptr!());
            } else {
                self.rotate_right(parent_ptr!());
            }

            link = self.root;
        }

        link.unwrap().as_mut().color = Color::Black;
    }
}
