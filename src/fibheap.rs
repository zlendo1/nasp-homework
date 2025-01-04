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

impl<T: Ord> FibHeap<T> {
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

    pub fn decrease_key(&mut self, node: &mut Tree<T>, new_key: T) {
        if new_key > node.node {
            panic!("New key is greater than current key");
        }

        node.node = new_key;
        let mut parent = node.parent;

        while let Some(parent_ptr) = parent {
            unsafe {
                if node.node >= (*parent_ptr).node {
                    break;
                }

                std::ptr::swap(node, &mut *parent_ptr);
                parent = (*parent_ptr).parent;
            }
        }

        if node.parent.is_none() {
            Self::order_min(&mut self.roots);
        }
    }

    pub fn delete(&mut self, node: &mut Tree<T>) {
        self.decrease_key(node, self.roots.last().unwrap().node.clone());
        self.pop();
    }
}

impl<T: Ord> FromIterator<T> for FibHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::new();
        heap.extend(iter);
        heap
    }
}

impl<T: Ord> Extend<T> for FibHeap<T> {
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

impl<T> Tree<T> {
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

    fn decrease_key(&mut self, new_key: T) {
        if new_key > self.node {
            panic!("New key is greater than current key");
        }

        self.node = new_key;
        let mut parent = self.parent;

        while let Some(parent_ptr) = parent {
            unsafe {
                if self.node >= (*parent_ptr).node {
                    break;
                }

                std::ptr::swap(self, &mut *parent_ptr);
                parent = (*parent_ptr).parent;
            }
        }

        if self.parent.is_none() {
            // Reorder the roots in the heap
        }
    }

    fn delete(&mut self) {
        self.decrease_key(self.node.clone());
        // Remove the node from the heap
    }
}
