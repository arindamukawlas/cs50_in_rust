fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./substitution key");
    } else if *(&args[1].len()) != 26 {
        println!("Key must contain 26 characters.");
    } else {
        println!(
            "ciphertext: {}",
            cs50::get_string("plaintext: ")
                .unwrap()
                .chars()
                .map(|chr| substitute((&args[1]).to_uppercase().chars().collect(), chr))
                .collect::<String>()
        );
    }
}

fn substitute(key: Vec<char>, plaintext: char) -> char {
    if plaintext.is_uppercase() {
        (&key[(plaintext.to_ascii_uppercase() as usize) - 65]).to_ascii_uppercase()
    } else if plaintext.is_lowercase() {
        (&key[(plaintext.to_ascii_uppercase() as usize) - 65]).to_ascii_lowercase()
    } else {
        plaintext
    }
}
