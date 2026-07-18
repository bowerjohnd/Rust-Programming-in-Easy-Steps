use std::io;
use std::io::Write;
use std::str::FromStr;

pub fn prompted<T>(prompt: &str) -> Option<T> where T: FromStr {

    loop {
        print!("{prompt} : ");
        io::stdout().flush().expect("Failed to print prompt");

        let mut line = String::new();

        if let Err(e) = io::stdin().read_line(&mut line) {
            panic!("Error reading user input: {e}");
        }

        if line.trim().is_empty() {
            return None;
        }
        else {
            match line.trim().parse() {
                Ok(value) => return Some(value),
                    Err(_) => println!("Invalid entry. Try again.")
            }
        }
    }
}
pub fn prompted_no_blank<T>(prompt: &str) -> T where T: FromStr {
        
    loop {
        match prompted(prompt) {
            Some(value) => return value,
                None => println!("Entry may not be blank.")
        }
    }
}

