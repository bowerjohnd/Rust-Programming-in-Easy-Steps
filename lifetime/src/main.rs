//fn shorter(x: &str, y: &str) -> &str {
fn shorter<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    }
    else {
        y
    }
}

fn main() {
    let mut v = Vec::new();
    for i in [1,2000,3,14,5] {
        let s = format!("{i}");
        v.push(String::from(shorter("***", &s)));
    }
    println!("values = {:?}", v);
}