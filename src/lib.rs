use std::io::{Write};
use std::process::{Command, Stdio};

pub fn get_real_output<'a>(path: &'a str, input: &str) -> String {
    let mut child = Command::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().expect("Failed to start process");

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(input.as_bytes()).expect("Failed to write into stdin");
    drop(child_stdin);
    
    let output = child.wait_with_output().expect("Failed to get output of executable");

    String::from_utf8(output.stdout).unwrap()
}