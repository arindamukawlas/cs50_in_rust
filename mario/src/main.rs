use std::io;

fn main() {
    let height: u8;
    loop {
        print!("Height:");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut height).expect("Failed");
        let height: i32 = match height.trim().parse() {
            Ok(ht) => ht,
            Err(_) => continue,
        };  
        if !(height < 0 || height > 23) { n = height; break;}
    }
}
