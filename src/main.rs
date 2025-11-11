use pathsearch::find_executable_in_path;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut exit = true;
    let builtins: [&str; 3] = ["echo", "exit", "type"];

    while exit {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.trim() == "exit 0" {
            exit = false;
        } else if command.trim().contains(' ') {
            let mut com_arr = command.trim().splitn(2, ' ');
            let com = com_arr.next().unwrap_or_default();
            let term = com_arr.next().unwrap_or_default();
            match com {
                "echo" => println!("{}", term),
                "type" => {
                    if builtins.contains(&term) {
                        println!("{} is a shell builtin", term)
                    } else if let Some(exe_path) = find_executable_in_path(&term) {
                        println!("{} is {}", term, exe_path.display());
                    } else {
                        println!("{}: not found", term)
                    }
                }
                _ => {}
            }
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}
