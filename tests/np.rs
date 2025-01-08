use nasp_homework::np::{Statement, CNF};

#[test]
fn verify_true() {
    let mut statement = Statement::new(3);
    statement.set(vec![false, false, false]);

    let cnf = CNF::new(vec![vec![1, 2, -3]]);

    assert!(cnf.verify(&statement));
}

#[test]
fn verify_false() {
    let mut statement = Statement::new(3);
    statement.set(vec![false, false, false]);

    let cnf = CNF::new(vec![vec![1, 2, 3]]);

    assert!(!cnf.verify(&statement));
}

#[test]
fn result_true() {
    let cnf = CNF::new(vec![vec![1, 2, 3]]);

    assert!(cnf.result());
}

#[test]
fn result_false() {
    let cnf = CNF::new(vec![vec![1, 1, 1], vec![-1, -1, -1]]);

    assert!(!cnf.result());
}
