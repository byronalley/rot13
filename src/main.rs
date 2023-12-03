fn main() {
    let input = std::env::args().skip(1);

    let output = input
        .map(|s| s.chars().map(|c| rot13(c)).collect::<String>())
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
        'A'..='Z' => char::from_u32(((from as u32) - 64 + 13) % 26 + 64).unwrap_or(from),
        'a'..='z' => char::from_u32(((from as u32) - 96 + 13) % 26 + 96).unwrap_or(from),
        _ => from,
    }
}
