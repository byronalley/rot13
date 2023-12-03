fn main() {
    let input = std::env::args().skip(1);

    let output = input
        .map(|s| s.chars().map(|c| rot13(c)).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", output);
}

fn rot13(from: char) -> char {
    match from {
        'A'..='Z' => char::from_u32(((from as u32) - 64 + 13) % 26 + 96).unwrap_or(from),
        'a'..='z' => char::from_u32(((from as u32) - 96 + 13) % 26 + 96).unwrap_or(from),
        _ => from,
    }
}
