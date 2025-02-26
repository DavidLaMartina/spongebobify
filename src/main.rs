use std::env;
use std::borrow::Cow;
use std::fs;
use std::path::Path;
use arboard::Clipboard;
use image::{io::Reader as ImageReader, DynamicImage, Rgba, RgbaImage};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rusttype::{Font, Scale};

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
        let mut spongebob_img = match load_image() {
            Ok(spongebob_img) => spongebob_img,
            Err(e) => {
                eprintln!("Failed to load Spongebob image: {}", e);
                return;
            }
        };
        let font_path = "./assets/impact.ttf";
        if let Err(e) = draw_text_on_image(&mut spongebob_img, &alternating_case, font_path) {
            eprintln!("Failed to draw text on image: {}", e);
            return;
        }
        if let Err(e) = copy_image_to_clipboard(&spongebob_img) {
            eprintln!("Failed to copy Spongebob image to clipboard: {}", e);
        }
        // let _ = copy_image_to_clipboard(&spongebob_img);
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

fn load_image() -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let image_path = "./images/spongebob_mock.jpeg";
    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("Error: Image file {:?} does not exist.", path).into());
    }
    let img = ImageReader::open(path)?.decode()?;
    let rgb_img = img.to_rgba8();
    Ok(rgb_img)
}

fn copy_to_clipboard(input: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(input.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), input.to_owned());
    println!("Spongebobified text copied to the clipboard successfully!");
}

fn draw_text_on_image(img: &mut RgbaImage, text: &str, font_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let font_data = fs::read(font_path)?;
    let font = Font::try_from_vec(font_data).ok_or("Failed to load font")?;

    let scale = Scale { x: 50.0, y: 50.0 }; // Font size
    let text_color = Rgba([255, 255, 255, 255]); // White text

    let start_x = 50; // X position
    let start_y = 100; // Y position (baseline)

    for (i, c) in text.chars().enumerate() {
        let glyph = font.glyph(c)
            .scaled(scale)
            .positioned(rusttype::point(start_x as f32 + (i as f32 * 30.0), start_y as f32));

        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = (bb.min.x + x as i32) as u32;
                let py = (bb.min.y + y as i32) as u32;

                if px < img.width() && py < img.height() {
                    let pixel = img.get_pixel_mut(px, py);
                    let alpha = (v * 255.0) as u8;
                    *pixel = Rgba([text_color.0[0], text_color.0[1], text_color.0[2], alpha]);
                }
            });
        }
    }

    Ok(())
}

fn copy_image_to_clipboard(image: &RgbaImage) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = image.dimensions();
    let img_bytes = image.clone().into_raw();

    let mut clipboard= Clipboard::new()?;
    let _ = clipboard.set_image(arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::from(img_bytes),
    });

    println!("Spongebob image captured to the clipboard successfully!");
    Ok(())
}
