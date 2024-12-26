mod input;
use input::{get_input, number_check};
use rand::prelude::*;

fn main() {
    let mut active = true;
    while active {
        let mut counter = 0;
        let mut rng = thread_rng();
        let actual_number: i32 = rng.gen_range(1..11);

        loop {
            let user_input = get_input("Guess the number: ");
            let user_number = number_check(&user_input);
            if user_number != actual_number {
                println!("Try again!");
                counter += 1;
                if user_number < actual_number {
                    println!("The number is higher");
                } else {
                    println!("The number is lower");
                }
            } else {
                let real_counter = counter + 1;
                println!("You guessed correctly! in {} tries", real_counter);
                break;
            }
        }
        let replay = get_input("Do you want to play again? (y/n): ");
        if replay != "y" {
            active = false;
        }
    }
    println!("Thanks for playing!");
}
