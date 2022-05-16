use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        // expect is handling error if io::Result has an error reading the line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim is used cause stdin readline adds the content with an line break ex: \n or \r\n
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("Equals!");
                break;
            }
        }
    }
}
