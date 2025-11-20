
// use std::io;
// use std::cmp::Ordering;

// use rand::Rng;

// fn main() {
    
//         println!("Guess the number!");


//                 let secret_number = rand::random_range(1..=100);
//         println!("value stored in secret_number is {}", secret_number);

//         loop {

//         println!("Please input your guess:");

//         let mut guess = String::new();
//       //  io::stdin().read_line(&mut guess).expect("Failed to read line.");
//         io::stdin().read_line(&mut guess).expect("Failed to read line.");



//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//            Err(_) => {
//             println!("expecting a valid number to proceed forward");
//             continue},
//         };

//         println!("You guessed: {guess}");

//         match guess.cmp(&secret_number){
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }

//     }

//     }


use std::io;
use std::cmp;

use rand::Rng;

fn main(){
    println!("Welcome to guesssing game.");

    loop{
        let secret_number = rand::random_range(0..=100);
    println!("guess a number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("reading a line is not working fine");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue
    };

    // guess.cmp()


    }

}







