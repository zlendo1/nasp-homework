mod rbtree;

#[cfg(test)]
mod tests;

use std::io;

use rbtree::RbTree;

fn main() {
    let mut tree: RbTree<i32> = RbTree::new();

    loop {
        println!("\nMenu:");
        println!("0. Exit");
        println!("1. Insert new node");
        println!("2. Inorder print nodes");
        print!("\nEnter your choice: ");

        use std::io::Write;
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
                print!("Enter the new node: ");
                io::stdout().flush().unwrap();

                let mut element = String::new();
                io::stdin()
                    .read_line(&mut element)
                    .expect("Failed to read input");
                let element: i32 = match element.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid integer.");
                        continue;
                    }
                };

                tree.insert(element);
                println!("Node inserted successfully!");
            }
            "2" => {
                println!("\nNodes:");
                if tree.is_empty() {
                    println!("No nodes to display.");
                } else {
                    let elements = tree.inorder_traverse();
                    for element in elements.iter() {
                        print!("{} ", *element);
                    }
                    println!("")
                }
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
