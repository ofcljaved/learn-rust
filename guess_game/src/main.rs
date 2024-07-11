use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("GUESS THE NUMBER!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let guess:u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
        };
        println!("Your guess is {}", num);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal =>{ println!("You win!!"); break;},
        }
    }
}
