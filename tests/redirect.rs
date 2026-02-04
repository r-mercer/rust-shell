use std::fs;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn shell_cmd(input: &str) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_codecrafters-shell"))
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to run shell");
    let stdin = child.stdin.as_mut().unwrap();
    writeln!(stdin, "{}", input).unwrap();
    // attempt to exit after
    writeln!(stdin, "exit").unwrap();
    child.wait().expect("failed to wait");
}

#[test]
fn test_built_in_echo_redirection() {
    let outf = "redirect_test_b.txt";
    let _ = fs::remove_file(outf);
    shell_cmd(&format!("echo hello world > {}", outf));
    let contents = fs::read_to_string(outf).expect("file should exist");
    assert!(contents.contains("hello world"));
    let _ = fs::remove_file(outf);
}

#[test]
fn test_external_ls_redirection() {
    let outf = "redirect_test_ls.txt";
    let _ = fs::remove_file(outf);
    shell_cmd(&format!("ls > {}", outf));
    let contents = fs::read_to_string(outf).expect("file should exist");
    // Contents should mention at least Cargo.toml or src
    assert!(contents.contains("Cargo.toml") || contents.contains("src"));
    let _ = fs::remove_file(outf);
}
