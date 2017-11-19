extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("Welcome to the Guess Game");

	// Generate secret number
	let min = 1;
	let max = 101;
	let secret = rand(min, max);

	loop{
		println!("Please enter a number: ");
		let mut num = String::new();

		io::stdin().read_line(&mut num)
			.expect("Failed to read line");

		// println!("You guessed {}", num);

		let num: u32 = match num.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match num.cmp(&secret) {
			Ordering::Less => println!("Too Small"),
			Ordering::Greater => println!("Too Big"),
			Ordering::Equal => {
				println!("Equal");
				break;
			}
		}
	}
}

fn rand(min: u32, max: u32) -> u32 {
	rand::thread_rng().gen_range(min, max)
}
