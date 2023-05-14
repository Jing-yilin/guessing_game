use std::io;
// bringing a type called std::cmp::Ordering into scope from the standard library
use std::cmp::Ordering;
// The Rng trait defines methods that random number generators implement, 
// and this trait must be in scope for us to use those methods. 
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    // input loop
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        println!("You guessed: {}", guess);

        //compare guess to secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

