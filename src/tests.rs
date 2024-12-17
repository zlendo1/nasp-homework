use crate::rbtree::RbTree;

#[test]
fn test_insert() {
    let mut tree: RbTree<u32> = RbTree::new();

    assert!(tree.is_empty());

    let mut elements: Vec<u32> = vec![6, 11, 10, 2, 9, 7, 5, 13, 22, 27, 36, 12, 31];

    for element in elements.iter() {
        tree.insert(*element);
    }

    let result = tree.inorder_traverse();

    elements.sort();

    assert_eq!(result, elements);
}

#[test]
fn test_clear() {
    let mut tree: RbTree<u32> = RbTree::new();

    assert!(tree.is_empty());

    let elements: Vec<u32> = vec![6, 11, 10, 2, 9];

    for element in elements.iter() {
        tree.insert(*element);
    }

    tree.clear();

    let result = tree.inorder_traverse();

    assert_eq!(result.len(), 0)
}
