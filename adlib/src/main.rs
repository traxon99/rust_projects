//Create a madlib get verb, place and an animal
//e.g. I was <verb> inside the <place> but then the <animal> ate my hat!

use std::io;

fn main() {
    let mut user_verb = String::new();
    let mut user_place = String::new();
    let mut user_animal = String::new();
    println!("type a verb.");
	io::stdin()
		.read_line(&mut user_verb)
		.expect("Failed to read line");
	
	let user_verb: String = user_verb.trim().parse().expect("Please type a verb");
    
    println!("type a place.");
	io::stdin()
		.read_line(&mut user_place)
		.expect("Failed to read line");
	
	let user_place: String = user_place.trim().parse().expect("Please type a place");

    println!("type an animal");
	io::stdin()
		.read_line(&mut user_animal)
		.expect("Failed to read line");
	
	let user_animal: String = user_animal.trim().parse().expect("Please type a animal");

    println!("I was {} inside the {} but then the {} ate my hat!", user_verb, user_place, user_animal);
}

//finished
//main take aways: initialize whether the variable should be mutable or not, and learn the 'mini' functions that mess with inputs (e.g. trim, parse, expect, etc.)