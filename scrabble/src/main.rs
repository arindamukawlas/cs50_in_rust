fn main() {
    let player1 = get_points(&cs50::get_string("Player 1: ").unwrap());
    let player2 = get_points(&cs50::get_string("Player 2: ").unwrap());
    if player1 > player2 {
        println!("Player 1 wins!");
    } else if player2 > player1 {
        println!("Player 2 wins!");
    } else {
        println!("Tie!");
    }
}

fn get_points(word: &str) -> usize {
    let point_map = vec![
        1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
    ];
    let mut points: usize = 0;
    for chr in word.chars() {
        if chr.is_ascii_alphabetic() {
            points += &point_map[(chr.to_ascii_uppercase() as usize) - 65];
        }
    }
    points
}
