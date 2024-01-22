fn main() {
    // Prompt for Input
    let card_number = get_number();
    let total_digits = (card_number.checked_ilog10().unwrap_or(0) + 1) as i64;
    if luhn(4003600000000014, total_digits) {
        if total_digits == 15
            && get_digit(card_number, total_digits as u32) == 3
            && (get_digit(card_number, (total_digits - 1) as u32) == 4
                || get_digit(card_number, (total_digits - 1) as u32) == 7)
        {
            println!("AMEX");
        } else if total_digits == 16
            && get_digit(card_number, total_digits as u32) == 5
            && get_digit(card_number, (total_digits - 1) as u32) > 0
            && get_digit(card_number, (total_digits - 1) as u32) < 6
        {
            println!("MASTERCARD");
        } else if (total_digits == 13 || total_digits == 16)
            && get_digit(card_number, total_digits as u32) == 4
        {
            println!("VISA");
        } else {
            println!("INVALID");
        }
    } else {
        println!("INVALID");
    }
}

fn get_number() -> i64 {
    let number: i64;
    loop {
        if let Ok(n) = cs50::get_i64("Number: ") {
            number = n;
            break;
        }
    }
    number
}

fn get_digit(number: i64, digit: u32) -> i64 {
    (number % 10_i64.pow(digit)) / 10_i64.pow(digit - 1)
}

fn luhn(number: i64, total_digits: i64) -> bool {
    let mut checksum: i64 = 0;
    for curr_digit in (2..=total_digits).step_by(2) {
        checksum += match get_digit(number, curr_digit as u32) {
            9 => 9,
            8 => 7,
            7 => 5,
            6 => 3,
            5 => 1,
            other => other * 2,
        } as i64
    }
    for curr_digit in (1..=total_digits).step_by(2) {
        checksum += get_digit(number, curr_digit as u32);
    }
    checksum % 10 == 0
}
