use num::traits::bounds::Bounded;

pub struct FibHeap<T> {
    roots: Vec<Tree<T>>,
    len: usize,
}

struct Tree<T> {
    node: T,
    children: Vec<Tree<T>>,
    parent: Option<*mut Tree<T>>,
    mark: bool,
}

impl<T: Ord + Bounded + Copy + Clone> FibHeap<T> {
    pub fn new() -> Self {
        Self {
            roots: Default::default(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn peek(&self) -> Option<&T> {
        self.roots.last().map(Tree::root)
    }

    pub fn push(&mut self, item: T) {
        let new_min = self.peek().map(|o| &item <= o).unwrap_or(true);

        self.roots.push(Tree::new(item));

        if !new_min {
            let i = self.roots.len() - 1;
            self.roots.swap(i - 1, i);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let Tree {
            node, mut children, ..
        } = match self.roots.pop() {
            Some(x) => x,
            None => return None,
        };

        self.len -= 1;

        for child in &mut children {
            child.parent = None;
        }

        self.roots.extend(children);

        Self::rebalance(&mut self.roots, self.len);
        Self::order_min(&mut self.roots);

        Some(node)
    }

    fn rebalance(roots: &mut Vec<Tree<T>>, nodes: usize) {
        if roots.is_empty() {
            return;
        }

        let cap = nodes.ilog2() + 1;

        let mut buf: Vec<Option<Tree<T>>> =
            std::iter::repeat_with(|| None).take(cap as usize).collect();

        while let Some(mut tree) = roots.pop() {
            loop {
                let degree = tree.degree();

                debug_assert!(
                    degree < cap as usize,
                    "Degree is greater than log2(len) + 1"
                );

                tree = match buf[degree].take() {
                    None => {
                        buf[degree] = Some(tree);
                        break;
                    }
                    Some(tree_b) if tree.root() <= tree_b.root() => {
                        tree.children.push(tree_b);
                        tree
                    }
                    Some(mut tree_b) => {
                        tree_b.children.push(tree);
                        tree_b
                    }
                }
            }
        }

        roots.extend(buf.into_iter().filter_map(|x| x));
    }

    fn order_min(roots: &mut Vec<Tree<T>>) {
        let min_index = roots
            .iter()
            .enumerate()
            .min_by_key(|(_, t)| t.root())
            .map(|(idx, _)| idx);

        if let Some(idx) = min_index {
            let lastidx = roots.len() - 1;
            roots.swap(idx, lastidx);
        }
    }

    pub fn union(mut first: Self, mut second: Self) -> Self {
        let mut new = Self::new();

        let i = first.roots.len();
        let j = second.roots.len();

        let swap_min = i != 0 && j != 0 && first.peek() < second.peek();

        new.roots.append(&mut first.roots);
        new.roots.append(&mut second.roots);

        new.len = first.len + second.len;
        first.len = 0;
        second.len = 0;

        if swap_min {
            new.roots.swap(i - 1, i + j - 1);
        }

        new
    }

    fn decrease_key(&mut self, tree_ptr: *mut Tree<T>, new_value: T) {
        unsafe {
            let tree = &mut *tree_ptr;

            debug_assert!(
                new_value <= tree.node,
                "New value must be less than the current value"
            );

            tree.node = new_value;

            if let Some(parent_ptr) = tree.parent {
                let parent = &mut *parent_ptr;

                if tree.node < parent.node {
                    self.cut(tree_ptr);
                    self.cascading_cut(parent_ptr);
                }
            }

            Self::order_min(&mut self.roots);
        }
    }

    fn delete(&mut self, tree_ptr: *mut Tree<T>) {
        self.decrease_key(tree_ptr, T::min_value());
        self.pop();
    }

    fn cut(&mut self, tree_ptr: *mut Tree<T>) {
        unsafe {
            let tree = &mut *tree_ptr;

            if let Some(parent_ptr) = tree.parent {
                let parent = &mut *parent_ptr;

                let index = parent
                    .children
                    .iter()
                    .position(|child| std::ptr::eq(child, tree));

                if let Some(idx) = index {
                    parent.children.swap_remove(idx);
                }

                tree.parent = None;
                tree.mark = false;
                self.roots.push(tree.take());
            }
        }
    }

    fn cascading_cut(&mut self, tree_ptr: *mut Tree<T>) {
        unsafe {
            let tree = &mut *tree_ptr;

            if let Some(parent_ptr) = tree.parent {
                if tree.mark {
                    self.cut(tree_ptr);
                    self.cascading_cut(parent_ptr);
                } else {
                    tree.mark = true;
                }
            }
        }
    }
}

impl<T: Ord + Bounded + Copy + Clone> FromIterator<T> for FibHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::new();
        heap.extend(iter);
        heap
    }
}

impl<T: Ord + Bounded + Copy + Clone> Extend<T> for FibHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();

        if let (_, Some(upr)) = iter.size_hint() {
            self.roots.reserve(upr);
        }

        for x in iter {
            self.push(x);
        }
    }
}

impl<T: Copy + Clone> Tree<T> {
    fn new(item: T) -> Self {
        return Self {
            node: item,
            children: Vec::new(),
            parent: None,
            mark: false,
        };
    }

    fn root(&self) -> &T {
        &self.node
    }

    fn degree(&self) -> usize {
        self.children.len()
    }

    fn take(&mut self) -> Tree<T> {
        let new_tree = Tree {
            node: self.node,
            children: std::mem::take(&mut self.children),
            parent: std::mem::take(&mut self.parent),
            mark: self.mark,
        };

        new_tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::*;

    #[quickcheck]
    fn decrease_key(mut elements: Vec<u32>) {
        if elements.is_empty() {
            return;
        }

        elements.sort();
        elements.dedup();
        elements.swap_remove(0);

        if elements.is_empty() {
            return;
        }

        let mut heap = FibHeap::new();

        for &element in &elements {
            heap.push(element);
        }

        let new_min = heap.peek().copied().unwrap() - 1;

        let min_node_ptr = heap.roots.last_mut().unwrap() as *mut Tree<u32>;
        heap.decrease_key(min_node_ptr, new_min);

        assert_eq!(heap.peek(), Some(&new_min))
    }

    #[quickcheck]
    fn delete(elements: Vec<u32>) {
        if elements.len() < 2 {
            return;
        }

        let mut heap = FibHeap::new();

        for &element in &elements {
            heap.push(element);
        }

        let delete_node_ptr = heap.roots.first_mut().unwrap() as *mut Tree<u32>;

        heap.delete(delete_node_ptr);

        assert_eq!(heap.len(), elements.len() - 1)
    }
}
