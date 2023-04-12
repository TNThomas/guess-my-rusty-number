use std::env::args;

pub struct Arguments {
    name:String,
    pub help:bool,
    pub guesses:i8,
    pub bad_arg:String
}

pub fn parse_args() -> Arguments {
    let mut parsed_args = Arguments {
        name: "".to_owned(),
        help: false,
        guesses: 100,
        bad_arg: "".to_owned()
    };
    for arg in args() {
        let mut parsed = arg.split('=');
        match parsed.next() {
            None => continue,
            Some(keyword) => {
                match keyword {
                    "help" => {
                        parsed_args.help = true;
                    },
                    "guesses" => {
                        parsed_args.guesses = match parsed.next() {
                            Some(guesses) => {
                                match guesses.parse::<i8>() {
                                    Ok(num_guesses) => {
                                        if (num_guesses > 0)
                                        && (num_guesses < 100) {
                                            num_guesses
                                        } else {
                                            -1
                                        }
                                    },
                                    Err(_) => -1
                                }
                            },
                            None => -1
                        }
                    },
                    _ => {
                        let owned_key: String = keyword.to_owned();
                        if parsed_args.name == "" {
                            parsed_args.name = owned_key
                        }
                        else {
                            // Invalid argument
                            parsed_args.bad_arg = owned_key;
                        }
                    }
                }
            }
        }
    }
    return parsed_args;
}
