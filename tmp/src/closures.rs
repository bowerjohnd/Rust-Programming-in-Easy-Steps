// Subject: Closures

//fn add(a: i32, b:i32) -> i32 {
//    a + b
//}
//let add = | a: i32, b: i32 | -> i32 {a + b};
// the following is the same as above
//let add = |a, b| a + b;

fn main() {
    let mut count = 0;

    let arr = [true, true, false];

    let s = arr.into_iter()
        .map (|_x| {count = count + 1; count});

    for c in s {
        println!("{c}");
    }
println!("There are {count} elements");
}