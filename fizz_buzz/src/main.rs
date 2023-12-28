// To read input from terminal
use std::io;

// Fixed parameters: when to Fizz and when to Buzz
const N1: u32 = 3;
const N2: u32 = 5;

    // SOLUTION 1: Via for loop

// const MAX: u32 = 101;
// fn main() {
//     for n in 1..MAX {
//         let mut output = String::new();
//         
//         if n % N1 == 0 {
//             output.push_str("Fizz")
//         } 
//         if n % N2 == 0 {
//             output.push_str("Buzz")
//         }
//         if output.is_empty() {
//             output.push_str(&n.to_string())
//         }
// 
//         println!("{output}");
//     }
// }

    // SOLUTION 2: Via user input

fn main() {
    // Infinite loop over terminal inputs
    loop {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input :(");
    
        // trim() is to remove potential white spaces
        // If there is an invalid input (i.e. 2nd arm), then we ask for another input
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let fb = fizz_buzz(input);
        println!("\t I declare: {fb} \n")
    }
}

fn fizz_buzz(n: u32) -> String {
    match (n % N1, n % N2) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        (_, _) => n.to_string()
    }
}
