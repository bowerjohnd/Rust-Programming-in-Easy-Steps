use std::io;
use std::io::Write;

// ----- INSERT EACH SUCCESSiVE FUNCTION AFTER THIS LINE

fn input_str(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to print prompt");

    let mut line = String::new();

    match io::stdin().read_line(&mut line) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to read line. {e}");
            return None;
        }
    }

    Some(line.trim().to_string())
}

fn input_number(prompt: &str) -> Option<f64> {
    
    let Some(contents) = input_str(prompt) else {
        return None;
    };

    match contents.parse() {
        Ok(value) => {
            return Some(value);
        },
        Err(_) => {
            return None;
        }
    }
}

fn input_array() -> [f64; 100] {
    
    let mut array = [0.0; 100];
    let mut next = 0;

    while let Some(element) = input_number("Number: ") {
        array[next] = element;
        next += 1;
        if next >= array.len(){
            break
        }
    }
    array
}

fn main() {

    let array = input_array();
    let mut total = 0.0;
    for num in array {
        total += num;
    }
    println!("Total = {total}");
}
