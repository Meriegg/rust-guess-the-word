mod args;

fn main() {
    // Check for enough arguments
    if args::get().len() < 2 {
        println!("Please provide a word! `cargo run WORD`");
        return;
    }

    const TRIES_LIMIT: u8 = 0;
    let secret_word: &String = &args::get()[1];
    let mut tries_left: u8 = 3;
    let mut has_won: bool = false;
    let mut guess = String::with_capacity(secret_word.len());

    while tries_left > TRIES_LIMIT {
        // Reset the string after every attempt
        guess = String::from("");

        println!("Please enter your guess down below: ");
        std::io::stdin().read_line(&mut guess).ok().expect("Could not read from `STDIN` !");

        if guess.trim() != secret_word.trim() {
            tries_left -= 1;
            println!("Wrong, You have {} tries left!", tries_left);
        } else {
            has_won = true;
            break;
        }
    }

    if has_won {
        println!("You WIN!");
    } else {
        println!("You lose :(");
    }
}
