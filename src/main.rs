use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing The Number Game! (1-100)");

    // generate random
    let secret_number = rand::thread_rng().gen_range(1..101);

    // get input till player wins
    loop {
        // a mutable string
        let mut guess = String::new();
        println!("Enter Your Input:");

        // get input
        io::stdin().read_line(&mut guess).expect("Failed to read!");

        // parse string to int, if it ain't convertable then start loop again
        // guess a new varible with same name but diff type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // compare guess with secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break; // stoping loop as we have won the game at this point
            }
        }
    }
}

/* -- About 'match' keyword --
A match expression is made up of arms. An arm consists of a pattern to match against,
and the code that should be run if the value given to match fits that arm’s pattern.
Rust takes the value given to match and looks through each arm’s pattern in turn.
Patterns and the match construct are powerful Rust features that let you express a
variety of situations your code might encounter and make sure that you handle them all.
*/
