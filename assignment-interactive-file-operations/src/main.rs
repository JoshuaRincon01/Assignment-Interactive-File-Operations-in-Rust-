use std::io::{self, BufRead, Write};
use std::process::Command;

#[derive(Debug)]
enum FileOperation {
    List(String),           // directory path
    Display(String),        // file path
    Create(String, String), // file path + content
    Remove(String),         // file path
    Pwd,                    // print working directory
}

fn main() {
    println!("Welcome to the File Operations Program!\n");

    loop {
        display_menu();

        print!("Enter your choice (0-5): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!\n");
                continue;
            }
        };

        let operation = match choice {
            0 => {
                println!("\nGoodbye!");
                break;
            }
            1 => {
                let path = get_user_input("Enter directory path: ");
                FileOperation::List(path)
            }
            2 => {
                let path = get_user_input("Enter file path: ");
                FileOperation::Display(path)
            }
            3 => {
                let path = get_user_input("Enter file path: ");
                let content = get_user_input("Enter content: ");
                FileOperation::Create(path, content)
            }
            4 => {
                let path = get_user_input("Enter file path: ");
                FileOperation::Remove(path)
            }
            5 => FileOperation::Pwd,
            _ => {
                println!("Invalid choice! Please select 0-5.\n");
                continue;
            }
        };

        perform_operation(operation);
        println!(); // empty line for readability
    }
}

fn display_menu() {
    println!("File Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
    println!();
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn perform_operation(operation: FileOperation) {
    let result = match operation {
        FileOperation::List(dir) => {
            println!("Directory listing of '{}':", dir);
            Command::new("ls")
                .arg("-la")           // nicer output
                .arg(&dir)
                .status()
        }

        FileOperation::Display(file) => {
            println!("Contents of '{}':", file);
            Command::new("cat")
                .arg(&file)
                .status()
        }

        FileOperation::Create(path, content) => {
            let cmd = format!("echo '{}' > {}", content.replace("'", "'\\''"), path);
            let status = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("File '{}' created successfully.", path);
                    return;
                }
                Ok(_) => eprintln!("Command failed with non-zero exit code"),
                Err(e) => eprintln!("Failed to execute command: {}", e),
            }
            return;
        }

        FileOperation::Remove(file) => {
            let status = Command::new("rm")
                .arg("-f")   
                .arg(&file)
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("File '{}' removed successfully.", file);
                    return;
                }
                Ok(_) => eprintln!("Command failed with non-zero exit code"),
                Err(e) => eprintln!("Failed to execute command: {}", e),
            }
            return;
        }

        FileOperation::Pwd => Command::new("pwd").status(),
    };

    match result {
        Ok(status) if status.success() => {
        }
        Ok(status) => {
            eprintln!("Command failed with exit code: {}", status.code().unwrap_or(-1));
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}