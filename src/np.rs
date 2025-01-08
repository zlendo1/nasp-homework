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

        let mut i = 0_usize;
        let max_iter = 2_usize.pow(self.num_variables as u32);

        while i < max_iter {
            if self.verify(&statement) {
                println!("{:?}", statement.binary);
                return true;
            }

            statement.increment();
            i += 1;
        }

        return false;
    }
}
