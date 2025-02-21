use std::env;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <your_string>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let alternating_case = alternate_case(input);
    copy_to_clipboard(&alternating_case);
    println!("{}", alternating_case);
}

fn alternate_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    assert_eq!(result.capacity(), input.len());
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

fn copy_to_clipboard(input: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(input.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), input.to_owned());
}
