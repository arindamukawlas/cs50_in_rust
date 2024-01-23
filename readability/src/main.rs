fn main() {
    let text = cs50::get_string("Text: ").unwrap();
    let words = get_words(&text);
    let index = 0.0588 * (get_letters(&text) * 100.0 / words)
        - 0.296 * (get_sentences(&text) * 100.0 / words)
        - 15.8;
    if index < 1.0 {
        println!("Before Grade 1");
    } else if index > 16.0 {
        println!("Grade 16+");
    } else {
        println!("Grade {}", index.round() as u8);
    }
}

fn get_letters(text: &str) -> f64 {
    let mut letters = 0;
    for chr in text.chars() {
        if chr.is_alphabetic() {
            letters += 1;
        }
    }
    letters as f64
}

fn get_words(text: &str) -> f64 {
    let mut words = 1;
    for chr in text.chars() {
        if chr.is_whitespace() {
            words += 1;
        }
    }
    words as f64
}

fn get_sentences(text: &str) -> f64 {
    let mut sentences = 0;
    for chr in text.chars() {
        if matches!(chr, '!' | '.' | '?') {
            sentences += 1;
        }
    }
    sentences as f64
}
