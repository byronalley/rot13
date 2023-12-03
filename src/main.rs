fn main() {
    let input = std::env::args().skip(1);

    let output = input
        .map(|s| s.chars().map(rot13).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", output);
}

#[test]
fn test_rot13_conversion() {
    assert_eq!(rot13('A'), 'N');
    assert_eq!(rot13('a'), 'n');
    assert_eq!(rot13('Z'), 'M');
    assert_eq!(rot13('z'), 'm');
    assert_eq!(rot13('!'), '!');
}

fn rot13(from: char) -> char {
    match from {
        'A'..='Z' => ((from as u8 - b'A' + 13) % 26 + b'A') as char,
        'a'..='z' => ((from as u8 - b'a' + 13) % 26 + b'a') as char,
        _ => from,
    }
}
