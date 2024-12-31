use std::collections::LinkedList;

pub struct FHeap<T> {
    roots: LinkedList<Tree<T>>,
    len: usize,
}

struct Tree<T> {
    node: T,
    children: Vec<Tree<T>>,
}

impl<T: Ord> FHeap<T> {
    pub fn peek(&self) -> Option<&T> {
        self.roots.front().map(Tree::root)
    }

    pub fn push(&mut self, item: T) {
        if self.peek().map(|o| &item <= o).unwrap_or(true) {
            self.roots.push_front(Tree::new(item));
        } else {
            self.roots.push_back(Tree::new(item));
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let Tree { node, children } = match self.roots.pop_front() {
            Some(x) => x,
            None => return None,
        };

        self.len -= 1;

        self.roots.extend(children);

        Self::rebalance(&mut self.roots, self.len);

        Self::bring_min_to_front(&mut self.roots);

        Some(node)
    }

    fn rebalance(roots: &mut LinkedList<Tree<T>>, nodes: usize) {
        if roots.is_empty() {
            return;
        }

        let cap = nodes.ilog2() + 1;

        let mut buf: Vec<Option<Tree<T>>> =
            std::iter::repeat_with(|| None).take(cap as usize).collect();

        while let Some(mut tree) = roots.pop_front() {
            loop {
                let degree = tree.degree();

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

    fn bring_min_to_front(roots: &mut LinkedList<Tree<T>>) {
        let min_index = roots
            .iter()
            .enumerate()
            .min_by_key(|(_, t)| t.root())
            .map(|(idx, _)| idx);

        if let Some(idx) = min_index {
            let mut split = roots.split_off(idx);
            split.append(roots);
            *roots = split;
        }
    }
}

impl<T> Tree<T> {
    fn new(item: T) -> Self {
        return Self {
            node: item,
            children: vec![],
        };
    }

    fn root(&self) -> &T {
        &self.node
    }

    fn degree(&self) -> usize {
        self.children.len()
    }
}
