use std::collections::HashMap;

pub struct Statement {
    binary: Vec<bool>,
}

pub struct CNF {
    formula: Vec<Vec<i32>>,
    num_variables: usize,
}

impl Statement {
    pub fn new(len: usize) -> Self {
        debug_assert!(len <= 24, "No more than 24 logical variables can exist!");

        return Self {
            binary: vec![false; len],
        };
    }

    pub fn set(&mut self, binary: Vec<bool>) -> &Self {
        debug_assert!(
            binary.len() == self.binary.len(),
            "New statement must be of same length!"
        );

        self.binary = binary;

        self
    }

    pub fn len(&self) -> usize {
        self.binary.len()
    }

    pub fn increment(&mut self) {
        for bit in self.binary.iter_mut() {
            if *bit == false {
                *bit = true;
                break;
            }

            *bit = false;
        }
    }
}

impl CNF {
    pub fn new(new_formula: Vec<Vec<i32>>) -> Self {
        debug_assert!(new_formula.len() < 8, "No more than 8 clauses permitable!");

        for row in new_formula.iter() {
            debug_assert!(row.len() == 3, "Every clause must contain three literals!")
        }

        let (formula, num_variables) = Self::normalize_matrix(new_formula);

        return Self {
            formula,
            num_variables,
        };
    }

    pub fn normalize_matrix(matrix: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, usize) {
        let mut unique_elements: Vec<i32> = matrix
            .iter()
            .flat_map(|row| row.iter())
            .map(|&val| val.abs())
            .collect();
        unique_elements.sort_unstable();
        unique_elements.dedup();

        let mut value_map: HashMap<i32, i32> = HashMap::new();
        for (new_value, &old_value) in unique_elements.iter().enumerate() {
            debug_assert!(
                old_value <= 24,
                "No more than 24 logical variables can exist!"
            );

            value_map.insert(old_value, (new_value as i32) + 1);
        }

        (
            matrix
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|val| {
                            let normalized = value_map[&val.abs()];
                            if val < 0 {
                                -normalized
                            } else {
                                normalized
                            }
                        })
                        .collect()
                })
                .collect(),
            value_map.len(),
        )
    }

    pub fn verify(&self, statement: &Statement) -> bool {
        for row in self.formula.iter() {
            if !row.iter().any(|x| {
                x.clone() > 0 && statement.binary[(x.clone() as usize) - 1]
                    || x.clone() < 0 && !statement.binary[(x.abs().clone() as usize) - 1]
            }) {
                return false;
            }
        }

        return true;
    }

    pub fn result(&self) -> bool {
        let mut statement = Statement::new(self.num_variables);

        let max_iter = 2_usize.pow(self.num_variables as u32);

        for _ in 0..max_iter {
            if self.verify(&statement) {
                println!("{:?}", statement.binary);
                return true;
            }

            statement.increment();
        }

        return false;
    }
}

pub struct Graph {
    relation: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(relation: Vec<Vec<usize>>) -> Self {
        let num_nodes = relation.len();

        for row in relation.iter() {
            debug_assert!(row.len() == num_nodes, "Relation matrix must be square!");

            for el in row.iter() {
                debug_assert!(
                    *el < num_nodes,
                    "Relations in matrix must relate to existing node!"
                );
            }
        }

        return Self { relation };
    }

    fn comp_size(&self, nodes: &Vec<usize>) {
        debug_assert!(
            nodes.len() <= self.relation.len(),
            "List of nodes must be compatible with relation matrix!"
        );
    }

    pub fn num_nodes(&self) -> usize {
        return self.relation.len();
    }

    pub fn verify_indset(&self, nodes: &Vec<usize>) -> bool {
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let u = nodes[i];
                let v = nodes[j];

                if self.relation[u][v] != 0 {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn verify_clique(&self, nodes: &Vec<usize>) -> bool {
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let u = nodes[i];
                let v = nodes[j];

                if self.relation[u][v] == 0 {
                    return false;
                }
            }
        }

        return true;
    }

    fn combinations(&self, n: usize, k: usize) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        let mut temp = Vec::new();

        self.combinations_helper(0, n, k, &mut temp, &mut result);

        result
    }

    fn combinations_helper(
        &self,
        start: usize,
        n: usize,
        k: usize,
        temp: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if temp.len() == k {
            result.push(temp.clone());
            return;
        }

        for i in start..n {
            temp.push(i);
            self.combinations_helper(i + 1, n, k, temp, result);
            temp.pop();
        }
    }

    pub fn result_k_clique(&self, k: usize) -> bool {
        let n = self.relation.len();
        let subsets = self.combinations(n, k);

        for subset in subsets {
            if self.verify_clique(&subset) {
                return true;
            }
        }
        false
    }

    pub fn result_k_indset(&self, k: usize) -> bool {
        let n = self.relation.len();
        let subsets = self.combinations(n, k);

        for subset in subsets {
            if self.verify_indset(&subset) {
                return true;
            }
        }
        false
    }

    pub fn to_indset(cnf: &CNF) -> Self {
        let mut nodes = Vec::new();
        let mut literal_to_node = HashMap::new();
        let mut next_node = 0;

        for clause in cnf.formula.iter() {
            for &literal in clause {
                if !literal_to_node.contains_key(&literal) {
                    literal_to_node.insert(literal, next_node);
                    nodes.push(literal);
                    next_node += 1;
                }
            }
        }

        let node_count = nodes.len();
        let mut relation = vec![vec![0; node_count]; node_count];

        for i in 0..node_count {
            for j in (i + 1)..node_count {
                let literal1 = nodes[i];
                let literal2 = nodes[j];

                if literal1 == -literal2
                    || cnf
                        .formula
                        .iter()
                        .any(|clause| clause.contains(&literal1) && clause.contains(&literal2))
                {
                    relation[i][j] = 1;
                    relation[j][i] = 1;
                }
            }
        }

        return Self { relation };
    }

    pub fn to_clique(cnf: &CNF) -> Graph {
        let mut nodes = Vec::new();
        let mut literal_to_node = HashMap::new();
        let mut next_node = 0;

        for clause in cnf.formula.iter() {
            for &literal in clause {
                if !literal_to_node.contains_key(&literal) {
                    literal_to_node.insert(literal, next_node);
                    nodes.push(literal);
                    next_node += 1;
                }
            }
        }

        let node_count = nodes.len();
        let mut relation = vec![vec![0; node_count]; node_count];

        for i in 0..node_count {
            for j in (i + 1)..node_count {
                let literal1 = nodes[i];
                let literal2 = nodes[j];

                if literal1 != -literal2
                    && !cnf
                        .formula
                        .iter()
                        .any(|clause| clause.contains(&literal1) && clause.contains(&literal2))
                {
                    relation[i][j] = 1;
                    relation[j][i] = 1;
                }
            }
        }

        return Self { relation };
    }
}
