use std::io;

fn main() {
    let mut input = String::new();
    let mut frequency: i32 = 0;
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(n) => {
                frequency += input.trim().parse::<i32>().unwrap();
                println!("Frequency update: {}", frequency);
                input.clear();
            }, Err(error) => eprintln!("error: {}", error),
        }
    }
}
