fn add (a: i32, b:i32) -> i32 {
	a + b
}

fn main() {
	let c = add(2,3);
	println!("2 + 3 = {c}");

	println!("4 + 5 = {0}", add(4,5));
}