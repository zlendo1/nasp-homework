pub struct FibHeap<T> {
    roots: Vec<Tree<T>>,
    len: usize,
}

struct Tree<T> {
    node: T,
    children: Vec<Tree<T>>,
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
        let Tree { node, children } = match self.roots.pop() {
            Some(x) => x,
            None => return None,
        };

        self.len -= 1;

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

        first.len = 0;
        second.len = 0;

        if swap_min {
            new.roots.swap(i - 1, j - 1);
        }

        new
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
        };
    }

    fn root(&self) -> &T {
        &self.node
    }

    fn degree(&self) -> usize {
        self.children.len()
    }
}
