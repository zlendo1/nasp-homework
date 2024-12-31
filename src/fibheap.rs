use std::collections::LinkedList;

pub struct FHeap<T> {
    roots: LinkedList<Tree<T>>,
}

struct Tree<T>(T);

impl<T: Ord> FHeap<T> {
    pub fn peek(&self) -> Option<&T> {
        self.roots.front().map(Tree::root)
    }
}

impl<T> Tree<T> {
    fn root(&self) -> &T {
        &self.0
    }
}
