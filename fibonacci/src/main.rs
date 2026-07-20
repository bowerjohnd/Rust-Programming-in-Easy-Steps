struct Fibonacci {
    previous: (u32, u32)
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci{previous: (1,0)}
    }
}

impl std::iter::Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.previous.0 + self.previous.1;
        self.previous = (self.previous.1, n);
        Some(n)
    }
}

fn main() {
    for i in Fibonacci::new() {
        println!("{i}");
        if i > 50 {break}
    }
}
