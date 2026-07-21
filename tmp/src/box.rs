// Example in book about Heap storage with Box
//      - Not meant to be a working example
//      - Reads an integer from a file, can return an error

use std::error::error;
#[derive(Debug)]
struct MyErr {
    s: String
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}, self.s")
    }
}

impl Error for MyErr {}

fn parse_line(f: &mut BufReader<File>)
    -> Result<i32, Box<dyn Error>> {

        let mut line = String::new();
        f.read_line(&mut line)?;
        let num: i32 = line.parse()?;

        if num < 10 || num > 100 {
            Err(Box::new(MyErr{s: String::from("Out of Range")}))
        }
        else {
            Ok(num)
        }
    }