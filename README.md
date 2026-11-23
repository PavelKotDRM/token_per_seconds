# Tokens per second simulator

A Rust and egui desktop application that demonstrates how prewritten text appears at a chosen speed in tokens per second. This is a simulation: it does not run an LLM or measure model performance.

## Running

Install Rust 1.95 or newer and the build tools for your operating system (Microsoft C++ Build Tools on Windows), then run:

```powershell
cargo run --release
```

On Windows, the release executable runs without opening a console window; debug builds retain the console for diagnostics.

The initial build requires access to crates.io. The built-in tokenizers and local files work offline afterward. On Linux, development libraries for the window system and graphics may be required; the file dialog uses the desktop portal.

## Usage

1. Select the built-in OpenAI `o200k_base` (the default, used by GPT-4o, for example) or `cl100k_base` (used by GPT-4, for example). Alternatively, open a local `tokenizer.json` in Hugging Face Tokenizers format. Other formats, model identifiers and automatic downloads are not supported.
2. Select an example or paste your own text in any language. The long examples repeat their passages so that playback remains visible at high speeds. Changing the text or tokenizer resets the simulation.
3. Click **Count tokens** to count tokens or **Start** to begin. The slider sets the speed from 1 to 1,500 tokens/s, including during playback. You can pause, resume, or restart.

Text is encoded by the selected tokenizer without automatically added special tokens. Saved truncation and padding settings are disabled for loaded tokenizers. The displayed text is the decoded token prefix: normalization, unknown tokens, and the tokenizer's decoder may change it from the input. A single character may span multiple tokens, so a replacement character can temporarily appear before all its bytes are available. Token counts depend on the text and tokenizer, not on the speed.

The examples cover several scripts, and custom text is not limited to particular languages. Glyph availability depends on installed system fonts. egui may render right-to-left text and complex shaping incorrectly (for example, Arabic or Devanagari); this display limitation does not affect tokenization.

## Verification

```powershell
cargo test
cargo fmt --check
```
