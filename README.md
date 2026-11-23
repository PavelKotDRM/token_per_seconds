# Tokens per second simulator

A Rust and egui desktop application that demonstrates how prewritten text appears at a chosen speed in tokens per second. This is a simulation: it does not run an LLM or measure model performance.

## Running

Install Rust 1.95 or newer and the build tools for your operating system (Microsoft C++ Build Tools on Windows), then run:

```powershell
cargo run --release
```

On Windows, the release executable runs without opening a console window; debug builds retain the console for diagnostics.

The initial build requires access to crates.io. The built-in tokenizers and local files work offline afterward. On Linux, development libraries for the window system and graphics may be required; the file dialog uses the desktop portal.

## Tabs

- **Texts**: browse and edit the text of each language, with its length (characters, UTF-8 bytes), linguistic length (words; Chinese and Japanese kana count per character) and token statistics per tokenizer. Edits update all tabs immediately; "Restore example" resets the text.
- **Compression efficiency**: bars for all languages together for one tokenizer (characters/bytes per token, tokens per word).
- **Tokenizer comparison**: grouped bars comparing all loaded tokenizers per language or across all languages.
- **Matrix**: languages × tokenizers heat map with optional first tokens of each text.
- **Simulator**: a matrix of live outputs, languages × tokenizers, all generated at the same tokens/s speed (slider, Start/Pause/Restart apply to every cell). Tokenizers are managed in the bar above the tabs: load several `tokenizer.json` files at once, switch between them, or remove the selected one. Once custom tokenizers are loaded, the built-in ones are collapsed under "Built-in tokenizers" and left out of all tabs unless you tick "Include built-in tokenizers in comparisons and simulation" there.

The **Export HTML report…** button in the tokenizer bar saves the statistics of the currently shown tokenizers as a standalone HTML page with Plotly charts (efficiency by language, tokenizer comparison, heat map) and a data table. The charts load Plotly from cdn.plot.ly, so viewing requires an internet connection.

## Usage

1. Select the built-in OpenAI `o200k_base` (the default, used by GPT-4o, for example) or `cl100k_base` (used by GPT-4, for example). Alternatively, open a local `tokenizer.json` in Hugging Face Tokenizers format. Other formats, model identifiers and automatic downloads are not supported.
2. Select an example or paste your own text in any language. The long examples repeat their passages so that playback remains visible at high speeds. Changing the text or tokenizer resets the simulation.
3. Click **Count tokens** to count tokens or **Start** to begin. The slider sets the speed from 1 to 1,500 tokens/s, including during playback. You can pause, resume, or restart.

Text is encoded by the selected tokenizer without automatically added special tokens. Saved truncation and padding settings are disabled for loaded tokenizers. The displayed text is the decoded token prefix: normalization, unknown tokens, and the tokenizer's decoder may change it from the input. A single character may span multiple tokens, so a replacement character can temporarily appear before all its bytes are available. Token counts depend on the text and tokenizer, not on the speed.

The examples cover several scripts, and custom text is not limited to particular languages. Glyph availability depends on installed system fonts. egui may render right-to-left text and complex shaping incorrectly (for example, Arabic or Devanagari); this display limitation does not affect tokenization.

## Releases and CI

Pull requests and pushes to `main` run `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` on Linux, Windows and macOS, and a RustSec dependency audit (`.github/workflows/ci.yml`).

Pushing a tag such as `v0.1.0` (it must match the version in `Cargo.toml`) runs the same checks, builds release binaries for Linux x86_64, Windows x86_64, and macOS x86_64 and aarch64, and publishes them to GitHub Releases with `SHA256SUMS.txt` (`.github/workflows/release.yml`). Every file receives a signed GitHub build provenance attestation. Verify a download with:

```powershell
gh attestation verify <file> --repo PavelKotDRM/token_per_seconds
```

The binaries are not code-signed with an operating system certificate, so Windows SmartScreen or macOS Gatekeeper may warn on first launch.

## Verification

```powershell
cargo test
cargo fmt --check
```
