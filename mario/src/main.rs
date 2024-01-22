fn main() {
    // Prompt for Input
    let height = get_height();

    // Print the Pyramid
    for y in 1..height {
        for x in 1..height {
            if x + y >= height {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("  {}", "#".repeat(y));
    }
}

fn get_height() -> usize {
    let height: usize;
    loop {
        if let Ok(n) = cs50::get_usize("Height: ") {
            if n > 0 && n < 9 {
                height = n + 1;
                break;
            }
        }
    }
    height
}
