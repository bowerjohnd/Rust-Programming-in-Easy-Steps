fn do_something (text: &mut String) {
	println!("Doing something with {text}");
	text.push_str(" now");
}

fn main() {
	let mut text = String::from("This is a string");
	do_something(&mut text);
	println!("Done something with {text}");
}