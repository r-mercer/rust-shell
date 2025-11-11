#[allow(unused_imports)]
use std::io::{self, Write};
// use std::string::StartsWith;

fn main() {
    let mut exit = true;

    while exit {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.trim() == "exit 0" {
            exit = false;
        } else if command.trim().contains(' ') {
            let mut com_arr = command.trim().splitn(2, ' ');
            if com_arr.next().unwrap() == "echo" {
                println!("{}", com_arr.next().unwrap())
            }
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}
