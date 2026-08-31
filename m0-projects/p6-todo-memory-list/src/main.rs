use std::io::{self, Write};
use p6_todo_memory_list::{Status, TodoList};

fn main() {
    let mut todo_list = TodoList::new();

    println!("=== Todo Memory List CLI ===");
    println!("Commands: add <title> | list | done <id> | remove <id> | exit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.splitn(2, ' ');
        let command = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("").trim();

        match command {
            "add" => {
                if args.is_empty() {
                    println!("[!] Error: Task title cannot be empty");
                } else {
                    let item = todo_list.add_item(args);
                    println!("[+] Added task: #{}: \"{}\"", item.id, item.title);
                }
            },
            "list" => {
                let items = todo_list.list_items();
                if items.is_empty() {
                    println!("List is empty");
                } else {
                    println!("======== TODO LIST ========");
                    for item in items {
                        let status_str = match item.status {
                            Status::Pending => "[Pending]",
                            Status::Done => "[Done]  ",
                        };
                        println!("[{}] {} {}", item.id, status_str, item.title);
                    }
                    println!("===========================");

                }
            },
            "done" => {
                if let Ok(id) = args.parse::<usize>() {
                    match todo_list.mark_done(id) {
                        Ok(()) => println!("[v] Task #{} marked as Done!", id),
                        Err(err) => println!("[!] {}", err),
                    }
                } else {
                    println!("[!] Error: Please provide a valid numeric ID")
                }
            },
            "remove" => {
                if let Ok(id) = args.parse::<usize>() {
                    match todo_list.remove_item(id) {
                        Some(item) => println!("[-] Removed task #{} \"{}\"", item.id, item.title),
                        None => println!("[!] Task with ID {} not found", id),
                    }
                } else {
                    println!("[!] Error: Please provide a valid numeric ID (e.g., 'remove 1')");
                }
            },
            "exit" => {
                println!("===========================");
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("[!] Unknown command: '{}'. Valid commands: add, list, done, remove, exit", command);
            }
        }

    }


}
