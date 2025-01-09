use std::io::{self, Write};

use nasp_homework::np::{Graph, Statement, CNF};

fn main() {
    let mut cnf: Option<CNF> = None;
    let mut graph: Option<Graph> = None;

    loop {
        println!("\nMenu:");
        println!("0. Exit");
        println!("1. Input formula");
        println!("2. Input graph");
        println!("3. Check formula satisfiability");
        println!("4. Check if k-indset exists");
        println!("5. Check if k-clique exists");
        println!("6. Verify formula");
        println!("7. Verify indset");
        println!("8. Verify clique");
        println!("9. Reduce 3-SAT to indset");
        println!("10. Reduce 3-SAT to clique");
        print!("\nEnter your choice: ");

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "0" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            "1" => {
                cnf = Some(input_cnf());
            }
            "2" => {
                graph = Some(input_graph());
            }
            "3" => {
                if let Some(f) = &cnf {
                    let satisfiable = f.result();

                    println!(
                        "Formula is {}satisfiable.",
                        if satisfiable { "" } else { "not " }
                    );
                } else {
                    println!("No formula found.");
                }
            }
            "4" => {
                if let Some(g) = &graph {
                    print!("Input k: ");

                    io::stdout().flush().unwrap();

                    let k = read_usize();
                    let result = g.result_k_indset(k);

                    println!(
                        "There {} a {}-indset in the graph.",
                        if result { "exists" } else { "does not exist" },
                        k
                    );
                } else {
                    println!("No graph found.");
                }
            }
            "5" => {
                if let Some(g) = &graph {
                    print!("Input k: ");

                    io::stdout().flush().unwrap();

                    let k = read_usize();
                    let result = g.result_k_clique(k);

                    println!(
                        "There {} a {}-clique in the graph.",
                        if result { "exists" } else { "does not exist" },
                        k
                    );
                } else {
                    println!("No graph found.");
                }
            }
            "6" => {
                if let Some(f) = &cnf {
                    let assignment = input_assignment();

                    let valid = f.verify(&assignment);

                    println!("The assignment is {}valid.", if valid { "" } else { "in" });
                }
            }
            "7" => {
                if let Some(g) = &graph {
                    let indset = input_set();
                    let valid = g.verify_indset(&indset);

                    println!("The set is {}independent.", if valid { "" } else { "not " });
                } else {
                    println!("No graph found.");
                }
            }
            "8" => {
                if let Some(g) = &graph {
                    let clique = input_set();
                    let valid = g.verify_clique(&clique);

                    println!("The set is {}a clique.", if valid { "" } else { "not " });
                } else {
                    println!("No graph found.");
                }
            }
            "9" => {
                if let Some(f) = &cnf {
                    graph = Some(Graph::to_indset(&f));
                } else {
                    println!("No formula found");
                }
            }
            "10" => {
                if let Some(f) = &cnf {
                    graph = Some(Graph::to_clique(&f));
                } else {
                    println!("No formula found");
                }
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

fn input_cnf() -> CNF {
    println!("Input formula (3-CNF):");

    let mut formula = Vec::new();

    loop {
        print!("Input clause (e.i. '1 -2 3 <Enter>', or <Enter> to end insertion): ");

        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();

        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let clause = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        formula.push(clause);
    }

    CNF::new(formula)
}

fn input_graph() -> Graph {
    println!("Input graph:");
    print!("Input the number of nodes: ");

    io::stdout().flush().unwrap();

    let n = read_usize();
    let mut relation = vec![vec![0; n]; n];

    println!("Input branch nodes (e.i. '1 2 <Enter>', or <Enter> to end insertion):");

    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();

        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let edge = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<usize>>();

        relation[edge[0]][edge[1]] = 1;
        relation[edge[1]][edge[0]] = 1;
    }

    Graph::new(relation)
}

fn input_assignment() -> Statement {
    println!("Input variable assignment:");

    let mut assignment = Vec::new();

    print!("Insert values (e.i. '1 0 1 <Enter>' for variables x1, x2, x3): ");

    io::stdout().flush().unwrap();

    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    assignment = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u8>().unwrap() != 0)
        .collect();

    Statement::from(assignment)
}

fn input_set() -> Vec<usize> {
    println!("Input node set:");
    print!("Input nodes (e.i. '1 2 3 <Enter>'): ");

    io::stdout().flush().unwrap();

    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect()
}

fn read_usize() -> usize {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap()
}
