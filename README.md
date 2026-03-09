# Interactive File Operations

A simple interactive command-line tool written in Rust that lets you perform basic file system operations by executing real shell commands (`ls`, `cat`, `rm`, `pwd`, etc.).

Built as a learning exercise to practice:

- Enums in Rust
- User input handling
- `std::process::Command`
- Basic error handling

## Features

- List files in a directory (`ls -la`)
- Display file contents (`cat`)
- Create a new file with custom content
- Remove files (`rm -f`)
- Print current working directory (`pwd`)
- Interactive menu (runs until you type 0)

## How to use

- First put "cd assignment-interactive-file-operations" in terminal
- Next "cargo build"  