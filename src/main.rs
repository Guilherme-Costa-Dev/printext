use leptess::LepTess;
use std::process::Command;

fn main() {
    let output = Command::new("hyprshot")
        .args(["-m", "region", "--raw"])
        .output()
        .unwrap();

    let img: Vec<u8> = output.stdout;

    if img.is_empty() {
        println!("Image was empty");
        return;
    }

    let mut ocr = LepTess::new(None, "por").expect("Failed to initialize Tesseract");
    ocr.set_image_from_mem(&img)
        .expect("Failed to read from memory and set img to ocr");
    let text = ocr.get_utf8_text().expect("Failed to get text from ocr");

    Command::new("wl-copy")
        .arg(text)
        .status()
        .expect("Failed to copy to clipboard with 'wl-copy' command");
}
