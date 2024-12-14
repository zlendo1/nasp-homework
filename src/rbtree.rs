use std::{borrow::Borrow, marker::PhantomData, ptr::NonNull};

enum Color {
    Black,
    Red,
}

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

pub enum Entry<'a, K: 'a> {
    Vacant(VacantEntry<'a, K>),
    Occupied(OccupiedEntry<'a, K>),
}

pub struct VacantEntry<'a, K: 'a> {
    tree: &'a mut RbTree<K>,
    parent: Link<K>,
    insert_pos: LinkPtr<K>,
    key: K,
    marker: PhantomData<&'a K>,
}

pub struct OccupiedEntry<'a, K: 'a> {
    tree: &'a mut RbTree<K>,
    node_ptr: NodePtr<K>,
    marker: PhantomData<&'a K>,
}

enum InsertPos<K> {
    Vacant {
        parent: Link<K>,
        link_ptr: LinkPtr<K>,
    },
    Occupied {
        node_ptr: NodePtr<K>,
    },
}

impl<K> Node<K> {
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

    fn reset_links(&mut self, parent: Link<K>) {
        self.parent = parent;
        self.left = None;
        self.right = None;
        self.color = Color::Red;
    }

    fn is_red(&self) -> bool {
        match self.color {
            Color::Black => false,
            Color::Red => true,
        }
    }

    fn is_black(&self) -> bool {
        match self.color {
            Color::Black => true,
            Color::Red => false,
        }
    }
}

pub struct RbTree<K> {
    root: Link<K>,
}

impl<K> RbTree<K>
where
    K: Copy,
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
            InsertPos::Occupied { node_ptr: _ } => false,
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
                    return InsertPos::Occupied { node_ptr };
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

                match parent {
                    None => break,
                    Some(mut parent_ptr) => {
                        if parent_ptr.as_ref().is_black() {
                            break;
                        }

                        let grandparent = parent_ptr.as_mut().parent;

                        match grandparent {
                            None => break,
                            Some(mut grandparent_ptr) => {
                                let left_uncle = grandparent_ptr.as_mut().left;
                                let right_uncle = grandparent_ptr.as_mut().right;

                                if parent == left_uncle {
                                    if let Some(mut right_uncle_ptr) = right_uncle {
                                        if right_uncle_ptr.as_ref().is_red() {
                                            parent_ptr.as_mut().color = Color::Black;
                                            right_uncle_ptr.as_mut().color = Color::Black;
                                            grandparent_ptr.as_mut().color = Color::Red;
                                            current = parent_ptr.as_mut().parent;
                                        }
                                    } else if node_ptr == parent_ptr.as_ref().right.unwrap() {
                                        current = node_ptr.as_mut().parent;
                                        self.rotate_left(node_ptr);
                                    } else {
                                        parent_ptr.as_mut().color = Color::Black;
                                        grandparent_ptr.as_mut().color = Color::Red;
                                        self.rotate_right(grandparent_ptr);
                                    }
                                } else {
                                    if let Some(mut left_uncle_ptr) = left_uncle {
                                        if left_uncle_ptr.as_ref().is_red() {
                                            parent_ptr.as_mut().color = Color::Black;
                                            left_uncle_ptr.as_mut().color = Color::Black;
                                            grandparent_ptr.as_mut().color = Color::Red;
                                            current = parent_ptr.as_mut().parent;
                                        }
                                    } else if node_ptr == parent_ptr.as_ref().left.unwrap() {
                                        current = node_ptr.as_mut().parent;
                                        self.rotate_right(node_ptr);
                                    } else {
                                        parent_ptr.as_mut().color = Color::Black;
                                        grandparent_ptr.as_mut().color = Color::Red;
                                        self.rotate_left(grandparent_ptr);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(mut node_ptr) = self.root {
                node_ptr.as_mut().color = Color::Black;
            }
        }
    }

    pub fn inorder_transverse(&self) -> Vec<K> {
        let mut result: Vec<K> = Vec::new();

        self.inorder(self.root.to_owned(), &mut result);

        result
    }

    fn inorder(&self, node: Link<K>, result: &mut Vec<K>) {
        match node {
            None => return,
            Some(node_ptr) => unsafe {
                let key = node_ptr.as_ref().key;

                self.inorder(node_ptr.to_owned().as_ref().left.to_owned(), result);
                result.push(key);
                self.inorder(node_ptr.to_owned().as_ref().left.to_owned(), result);
            },
        }
    }
}
