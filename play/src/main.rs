fn main() {
    let x = 5;
    let y = x;
    // x和y在栈上分配内存以存储数据
    println!("{} {}", x, y);

    let s1 = String::from("hello"); // 字符串存储在“堆”上
    //let s2 = s1; // s1 -> s2，发生浅拷贝，s1便不可用。mv operation
    
    // way1:
    let s2 = &s1;
    // way2
    let s3 = s1.clone();
    println!("{} {} {}", s1, s2, s3);
}

// struct Rectangle {
//     width: u8,
//     height: u8
// }

// impl Rectangle {
//     fn is_square(&self) -> bool {
//         self.width == self.height
//     }
// }

// fn give_two() -> i16 {
//     2
// }

// #[cfg(test)]
// mod dcode_tests {
//     #[test]
//     #[should_panic]
//     fn test_basic() {
//         assert!(1 == 1);
//         panic!("Oh no!");
//     }

//     #[test]
//     fn test_equals() {
//         assert_eq!(super::give_two(), 1 + 1);
//         assert_ne!(2, 1 + 2);
//     }

//     #[test]
//     fn test_structs() {
//         let r = super::Rectangle {
//             width: 50,
//             height: 50
//         };

//         assert!(r.is_square())
//     }

//     #[test]
//     #[ignore]
//     fn test_equals2() {
//         assert_eq!(2, 1 + 1);
//         assert_ne!(2, 1 + 2);
//     }
// }


// use std::process::Command;

// fn main() {
//     let mut cmd = Command::new("python3");
//     cmd.arg("hello.py");


//     // Execute
//     match cmd.output() {
//         Ok(o) => {
//             unsafe {
//                 println!("Output: {}", String::from_utf8_unchecked(o.stdout));
//             }
//         },
//         Err(e) => {
//             println!("There was an error! {}", e);
//         }
//     }
// }


// #![allow(dead_code)]

// enum Day {
//     Monday, Tuesday, Wednesday,
//     Thursday, Friday, Saturday, Sunday
// }


// impl Day {
//     fn is_weekday(&self) -> bool {
//         match self {
//             Day::Saturday | Day::Sunday => return false,
//             _ => return true
//         }
//     }
// }

// fn main() {
//     let d = Day::Tuesday;

//     println!("Is d a weekday? {}", d.is_weekday());
// }

// fn main() {
//     {
//         let my_string = String::from("Rust is fantastic!");
//         println!("After replace: {}", my_string.replace("fantastic", "great"));

//     }

//     {
//         let my_string = String::from("The weather is\nnice\noutside mat!");

//         for line in my_string.lines() {
//             println!("[ {} ]", line);
//         }
//     }

//     {
//         let my_string = String::from("Leave+a+like+if+you+enjoyed!");
//         let tokens: Vec<&str> = my_string.split("+").collect();

//         println!("{}", my_string);
//         println!("At index 2: {}", tokens[2]);

//     }

//     {
//         let my_string = String::from("  My name is SX \n");
//         println!("after trim: {}", my_string.trim());
//         println!("{}", match my_string.chars().nth(8) {
//             Some(c) => c,
//             None => 'N'
//         });
//     }
// }


// extern crate rand;
// use rand::Rng;

// fn main() {
//     let random_number = rand::thread_rng().gen_range(1, 11);
//     println!("Random number: {}", random_number);

//     let random_bool = rand::thread_rng().gen_weighted_bool(2);
//     println!("Random Boolean: {}", random_bool);
// }

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
