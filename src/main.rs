use rand::distributions::{Distribution, Uniform};

mod parse_args;

fn main() {
    let args: parse_args::Arguments = parse_args::parse_args();

    if args.bad_arg != "" {
        println!("Invalid argument: {}", args.bad_arg);
        return;
    }

    if args.guesses < 1 {
        println!("Invalid value for argument 'guesses': {}. Please choose an integer between 1 and 100.", args.guesses);
    }

    if args.help {
        // TODO: Print help message
    }

    let mut rng = rand::thread_rng();
    let selectable_range: Uniform<i8> = Uniform::from(1..101);

    let target: i8 = selectable_range.sample(&mut rng);
    println!("I've chosen a random integer between 1 and 100!");
    println!("Enter a guess and I'll tell you if my number is higher or lower.");
    let mut num_guesses: i8 = 1;
    while num_guesses < 100 {
        if args.guesses - num_guesses < 0 {
            println!("You ran out of guesses! My number was {}.", target);
            break;
        }
        else {
            println!("Guess {} of {}: ", num_guesses, args.guesses);
        }
        let mut guess_raw = String::new();
        match std::io::stdin().read_line(&mut guess_raw) {
            Err(err) => {
                panic!("Error: {}", err)
            },
            Ok(_) => {
                let guess: i8 = guess_raw.trim().parse().unwrap();
                if guess < target {
                    if guess < 1 {
                        println!("{} isn't between 1 and 100! I'm still counting it as a guess, though!", guess);
                    }
                    else {
                        println!("Good guess, but {} is too low!", guess);
                    }
                }
                else {
                    if guess == target {
                        println!("That's right! My number was {}!", target);
                        println!("You won in {} guesses.", num_guesses);
                        break;
                    }
                    else if guess > 100 {
                        println!("{} isn't between 1 and 100! I'm still counting it as a guess, though!", guess);
                    }
                    else {
                        println!("Good guess, but {} is too high!", guess);
                    }
                }
                num_guesses += 1;
            }
        }
    }
}
