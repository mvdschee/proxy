use std::env;

fn main() {
	let config = env::var("CONFIG");

	dbg!(config);

	println!("Hello, world!");
}
