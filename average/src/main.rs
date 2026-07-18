use std::fmt;
use average::{mean, mode, median, Average};

struct List<T> {
    data: Vec<T>
}

impl<T> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)
        -> fmt::Result {

        write!(f, "list of {} items", self.data.len())
    }
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::<T>{data: Vec::<T>::new()}
    }

    fn add(&mut self, datum: &mut Vec<T>) {
        self.data.append(datum);
    }

    fn push(&mut self, datum: T) {
        self.data.push(datum)
    }
}

impl Average<f64> for List<f64> 
{
    fn average(&self) -> f64{
        mean(&self.data)
    }
}

impl Average<u8> for List<u8> {
    fn average(&self) -> u8 {
        median(&self.data)
    }
}

impl Average<String> for List<String> {
    fn average(&self) -> String {
        mode(&self.data)
    }
}

fn main() {
    let mut heights = List::new();
    heights.add(&mut vec![1.2, 1.5, 1.4, 1.55, 1.7]);
    println!("We have a {heights}");
    println!("Average heights = {}", heights.average());

    let mut children = List::new();
    children.add(&mut vec![3, 0, 2, 1, 2, 3, 1, 2]);
    println!("Average children = {}", children.average());

    let mut eye_colors: List<String> = List::new();
    let colors = vec!["brown", "blue", "brown", "green", "brown", "blue"];
    for s in colors {
        eye_colors.push(s.into());
    }

    println!("Average eye color = {}", eye_colors.average());
}