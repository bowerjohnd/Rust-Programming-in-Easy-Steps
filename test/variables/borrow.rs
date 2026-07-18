fn main() {
	let mut x = 0;
	x = x + 1;
	let y = &mut x;
	*y = *y + 1;
	println!("Value {x}");
}