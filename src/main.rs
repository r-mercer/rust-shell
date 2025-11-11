#[allow(unused_imports)]
use std::io::{self, Write};
// use std::string::StartsWith;

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    // match command.trim() {
    //     "exit 0" => (),
    //     // Some(x) if x.starts => 
    //     command.trim().starts_with("echo ") => 
    //         "echo" => (),
    //     _ => {
    //         println!("{}: command not found", command.trim());
    //         main();
    //     }
    // }

    if command.trim() == "exit 0" {}
    else if command.trim().contains(' ') {
        let mut com_arr = command.trim().splitn(2, ' ');
        if com_arr.next().unwrap() == "echo"
        {
            println!("{}", com_arr.next().unwrap() )
        }
    }
    else {
        println!("{}: command not found", command.trim());
        main();
    }
}
