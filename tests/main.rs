use std::io::Write;
use std::process::{Command, Stdio};

const CMD: &str = "target/debug/rot13";

#[test]
fn rot13_args_test() {
    let args = vec!["ab", "cd"];

    let output = Command::new(CMD)
        .args(args)
        .output()
        .expect("Failed to run the program");

    let string = String::from_utf8_lossy(&output.stdout).trim().to_string();

    assert_eq!(string, "no pq");
}

#[test]
fn stdin_test() {
    let input = String::from("ab cd");

    let mut child = Command::new(CMD)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run the program");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "no pq");
}
