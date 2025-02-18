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
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_lowercase().collect::<String>()
            }
        })
        .collect::<String>()
}
