use nasp_homework::np::{Statement, CNF};

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
