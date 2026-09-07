# printext 

A lightning-fast, minimal Rust utility for Wayland/Hyprland that lets you select a screen region, performs Optical Character Recognition (OCR) on the fly, and instantly copies the extracted text to your clipboard.

Unlike other tools that save temporary files to your disk, `printext` pipes the raw image bytes directly from your screen into memory, processing it instantly.

##  Features
- **No Temporary Files:** Processes image data directly in RAM.
- **Wayland Native:** Built around `hyprshot` and `wl-clipboard`.
- **Multi-Language Support:** Defaults to English, but supports any Tesseract language model.

##  Prerequisites

Since this tool integrates closely with your system environment, you need to have the following installed:

- **Rust / Cargo:** To compile the project.
- **hyprshot:** For capturing the screen (requires `grim` and `slurp`).
- **wl-clipboard:** Specifically `wl-copy` to handle the clipboard data.
- **tesseract:** The core OCR engine.
- **tesseract language data:** e.g., `tesseract-data-eng` (English). You may need to install other language data if you would like to pass it as an argument.
- **leptonica:** Required by the `leptess` Rust crate.

### Arch Linux Install
```bash
sudo pacman -S rust hyprshot grim slurp wl-clipboard tesseract tesseract-data-eng leptonica
