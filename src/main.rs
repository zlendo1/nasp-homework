use std::io::{self, Write};

use nasp_homework::np::{Graph, CNF};

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
                    )
                } else {
                    println!("No graph found.")
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
                    )
                } else {
                    println!("No graph found.")
                }
            }
            "6" => {
                if let Some(f) = &cnf {
                    let assignment = input_assignment();

                    let valid = f.verify(assignment);

                    println!("The assignment is {}valid.", if valid { "" } else { "in" })
                }
            }
            "7" => {}
            "8" => {}
            "9" => {}
            "10" => {}
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
