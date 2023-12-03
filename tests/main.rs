use std::process::Command;

#[test]
fn rot13_args_test() {
    let args = vec!["ab", "cd"];

    let output = Command::new("target/debug/rot13")
        .args(args)
        .output()
        .expect("Failed to run the program");

    let string = String::from_utf8_lossy(&output.stdout).trim().to_string();

    assert_eq!(string, "no pq");
}
