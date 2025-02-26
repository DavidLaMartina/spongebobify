use std::env;
use std::path::{Path, PathBuf};
use arboard::Clipboard;
use image::{io::Reader as ImageReader, DynamicImage};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <your_string> <image or text>", args[0]);
        std::process::exit(1);
    }
    if args[2] != "image" && args[2] != "text" {
        eprintln!("Your second argument must be either image or text");
        std::process::exit(1);
    }

    let input = &args[1];
    let output_type = &args[2];
    let alternating_case = alternate_case(input);

    if output_type == "image" {
        let spongebob_img = load_image();
        let _ = copy_image_to_clipboard(&spongebob_img);
    } else {
        copy_to_clipboard(&alternating_case);
    }
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

fn load_image() -> DynamicImage {
    let image_path = "./images/spongebob_mock.jpeg";
    let path = Path::new(image_path);
    if !path.exists() {
        eprintln!("Error: Spongebob image file does not exist.");
        return Default::default();
    }
    let img: DynamicImage = ImageReader::open(path)
        .expect("Failed to open Spongebob image.")
        .decode()
        .expect("Failed to decode Spongebob image.");
    img
}

fn copy_to_clipboard(input: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(input.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), input.to_owned());
    println!("Spongebobified text copied to the clipboard successfully!");
}

fn copy_image_to_clipboard(image: &DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
    let img = image.to_rgba8();
    let (width, height) = img.dimensions();
    let img_bytes = img.into_raw();

    let mut clipboard= Clipboard::new()?;
    let _ = clipboard.set_image(arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::from(img_bytes),
    });

    println!("Spongebob image captured to the clipboard successfully!");
    Ok(())
}
