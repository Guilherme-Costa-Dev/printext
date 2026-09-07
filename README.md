# printext 

A lightning-fast, minimal Rust utility for Wayland/Hyprland that lets you select a screen region, performs Optical Character Recognition (OCR) on the fly, and instantly copies the extracted text to your clipboard.

Unlike other tools that save temporary files to your disk, `printext` pipes the raw image bytes directly from your screen into memory, processing it instantly.

## Features
- **No Temporary Files:** Processes image data directly in RAM.
- **Wayland Native:** Built around `hyprshot` and `wl-clipboard`.
- **Multi-Language Support:** Defaults to English, but supports any Tesseract language model.

## Prerequisites

Since this tool integrates closely with your system environment, you need to have the following installed:

- **Rust / Cargo:** To compile the project.
- **hyprshot:** For capturing the screen (requires `grim` and `slurp`).
- **wl-clipboard:** Specifically `wl-copy` to handle the clipboard data.
- **tesseract:** The core OCR engine.
- **tesseract language data:** e.g., `tesseract-data-eng` (English). You may need to install other language data if you would like to use it as an argument.
- **leptonica:** Required by the `leptess` Rust crate.

### Arch Linux Install
```bash
sudo pacman -S rust hyprshot grim slurp wl-clipboard tesseract tesseract-data-eng leptonica
```

## Installation

Clone the repository and install it globally via Cargo:

```bash
git clone [https://github.com/YOUR_USERNAME/printext.git](https://github.com/YOUR_USERNAME/printext.git)
cd printext
cargo install --path .
```

*(This will place the executable in `~/.cargo/bin/printext`. Make sure `~/.cargo/bin` is in your system's `$PATH`).*

## Usage

Run the `printext` command in your terminal.

```bash
printext
```

**Specific Language:**
You can pass the Tesseract language code as an argument. Make sure you have the respective language data installed in your OS.

```bash
printext por       # Portuguese
printext spa       # Spanish
printext por+eng   # Portuguese and English combined
```

## 📄 License

This project is licensed under the [MIT License](LICENSE).
