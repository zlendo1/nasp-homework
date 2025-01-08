use nasp_homework::np::{Graph, Statement, CNF};

#[test]
fn cnf_verify() {
    let mut statement = Statement::new(3);
    statement.set(vec![false, false, false]);

    let cnf1 = CNF::new(vec![vec![1, 2, -3]]);
    let cnf2 = CNF::new(vec![vec![1, 2, 3]]);

    assert!(cnf1.verify(&statement));
    assert!(!cnf2.verify(&statement));
}

#[test]
fn cnf_result() {
    let cnf1 = CNF::new(vec![vec![1, 2, 3]]);
    let cnf2 = CNF::new(vec![vec![1, 1, 1], vec![-1, -1, -1]]);

    assert!(cnf1.result());
    assert!(!cnf2.result());
}

#[test]
fn verify_indset() {
    let graph = Graph::new(vec![
        vec![0, 1, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![0, 0, 1, 0],
    ]);

    assert!(graph.verify_indset(&vec![0, 2]));
    assert!(graph.verify_indset(&vec![1, 3]));

    assert!(!graph.verify_indset(&vec![0, 1]));
    assert!(!graph.verify_indset(&vec![2, 3]));
}

#[test]
fn verify_clique() {
    let graph = Graph::new(vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 1, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 1, 0],
    ]);

    assert!(graph.verify_clique(&vec![1, 2]));
    assert!(graph.verify_clique(&vec![1, 2, 3]));

    assert!(!graph.verify_clique(&vec![0, 3]));
    assert!(!graph.verify_clique(&vec![0, 1, 3]));
}

#[test]
fn empty_singleton_set() {
    let graph = Graph::new(vec![vec![0, 1], vec![1, 0]]);

    assert!(graph.verify_indset(&vec![]));
    assert!(graph.verify_clique(&vec![]));

    assert!(graph.verify_indset(&vec![0]));
    assert!(graph.verify_clique(&vec![0]));
    assert!(graph.verify_indset(&vec![1]));
    assert!(graph.verify_clique(&vec![1]));
}

#[test]
fn result_k_indset() {
    let graph = Graph::new(vec![
        vec![0, 1, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![0, 0, 1, 0],
    ]);

    assert!(graph.result_k_indset(1));
    assert!(graph.result_k_indset(2));
    assert!(!graph.result_k_indset(3));
    assert!(!graph.result_k_indset(5));
}

#[test]
fn result_k_clique() {
    let graph = Graph::new(vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 1, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 1, 0],
    ]);

    assert!(graph.result_k_clique(2));
    assert!(graph.result_k_clique(3));
    assert!(!graph.result_k_clique(4));
    assert!(!graph.result_k_clique(5));
}

#[test]
fn graph_edge_cases() {
    let empty_graph = Graph::new(vec![]);

    assert!(!empty_graph.result_k_clique(1));
    assert!(!empty_graph.result_k_indset(1));

    let single_node_graph = Graph::new(vec![vec![0]]);

    assert!(single_node_graph.result_k_clique(1));
    assert!(single_node_graph.result_k_indset(1));
    assert!(!single_node_graph.result_k_clique(2));
    assert!(!single_node_graph.result_k_indset(2));
}
