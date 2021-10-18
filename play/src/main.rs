extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 11);
    println!("Random number: {}", random_number);

    let random_bool = rand::thread_rng().gen_weighted_bool(2);
    println!("Random Boolean: {}", random_bool);
}

// use std::collections::HashMap;

// fn main() {
//     let mut marks = HashMap::new();

//     marks.insert("Rust programming", 95);
//     marks.insert("XYZ", 92);
//     marks.insert("zzz", 25);
//     marks.insert("abc", 45);

//     println!("{}", marks.len());

//     match marks.get("Web develop") {
//         Some(mark) => println!("You got {} for Web Dev!", mark),
//         None => println!("You didn't study Web Dev!")
//     }

//     for (subject, mark) in &marks {
//         println!("for {} you got {}%!", subject, mark);
//     }

// }

// use std::io;

// fn main() {
//     let mut input = String::new();

//     println!("Hey mate! Say something:");

//     match io::stdin().read_line(&mut input) {
//         Ok(_) => {
//             println!("Success! You said: {}", input);
//         },
//         Err(e) => println!("Oops! Something went wrong: {}", e)
//     }
// }


// fn main() {
//     let number = 2;

//     match number {
//         1 => println!("It's one"),
//         2 => println!("There is two of them"),
//         _ => println!("It doesn't match")
//     }

//     for i in 1..=10 {
//         println!("{}", i)
//     }
// }

// struct Person {
//     name: String,
//     age: u8
// }

// trait HasVoiceBox {
//     // Speak
//     fn speak(&self);
//     // Check if can speak
//     fn can_speak(&self) -> bool;
// }

// impl HasVoiceBox for Person {
//     fn speak(&self) {
//         println!("My name is {}", self.name)
//     }

//     fn can_speak(&self) -> bool {
//         if self.age > 3 {
//             return true;
//         } return false;
//     }
// }

// fn main() {
//     let person = Person {
//         name: String::from("Bob"),
//         age: 10
//     };

//     println!("Can {} speak? {}", person.name, person.can_speak())
// }


// use std::fs::File;
// use std::io::prelude::*;

// fn main() {
//     let mut file = File::create("output.txt")
//         .expect("Could not create file!");

//     file.write_all(b"Welcome to decode!")
//         .expect("Cannot write to the file, sorry mate.");
// }


// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     for argument in args.iter() {
//         println!("{}", argument)
//     }
// }


// fn main() {
//     // let tup1 = (20, 25, 30, 35);
//     let tup1 = (20, "Rust", 3.4, false, (1, 4, 7));
//     println!("{}", (tup1.4).2)

//     let (a, b, c, d, e) = tup1;
// }


// const MAIMUM_NUMBER: u8 = 20;

// fn main() {
//     for n in 1..MAIMUM_NUMBER {
//         println!("{}", n)
//     }
// }
