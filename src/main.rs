// use ferris_says::say;
// use std::io::{stdout, BufWriter};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <your_string>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let alternating_case = alternate_case(input);
    println!("{}", alternating_case);
}

fn alternate_case(input: &str) -> String {
    let mut result = String::new();
    let mut upper_case = true;
    for c in input.chars() {
        if !c.is_alphabetic() {
            result.push(c);
        }
        else if upper_case {
            result.push(c.to_ascii_uppercase());
            upper_case = !upper_case;
        } else {
            result.push(c.to_ascii_lowercase());
            upper_case = !upper_case;
        }
    }
    result
}
