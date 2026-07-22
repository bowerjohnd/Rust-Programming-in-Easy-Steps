// examples in Chapter 12 of Advanced traits

type MyVec: (Vec<i32>);

impl Deref for MyVec {
    type Target: Vec<i32>;
    fn deref(&self) -> &Self::Target {&self, 0}
}

struct Name (String)
impl Default for Name {
    fn default() -> Self {
        Name("Richard".into())
    }
}

#[derive(Default)]
struct Point {x:f64, y:f64}