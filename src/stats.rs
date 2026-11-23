use crate::simulation::TokenizerKind;

const SAMPLE_TOKENS: usize = 8;

#[derive(Clone, Default)]
pub struct Stats {
    pub chars: usize,
    pub bytes: usize,
    pub words: usize,
    pub tokens: usize,
    pub sample: String,
}

impl Stats {
    pub fn measure(tokenizer: &TokenizerKind, text: &str) -> Result<Self, String> {
        let ids = tokenizer.encode(text)?;
        let sample = ids
            .iter()
            .take(SAMPLE_TOKENS)
            .map(|id| tokenizer.decode(&[*id]))
            .collect::<Result<Vec<_>, _>>()?
            .join("|");
        Ok(Self {
            chars: text.chars().filter(|c| !c.is_whitespace()).count(),
            bytes: text.len(),
            words: count_words(text),
            tokens: ids.len(),
            sample,
        })
    }

    pub fn chars_per_token(&self) -> f64 {
        ratio(self.chars, self.tokens)
    }

    pub fn bytes_per_token(&self) -> f64 {
        ratio(self.bytes, self.tokens)
    }

    pub fn tokens_per_word(&self) -> f64 {
        ratio(self.tokens, self.words)
    }
}

fn ratio(a: usize, b: usize) -> f64 {
    if b == 0 { 0.0 } else { a as f64 / b as f64 }
}

fn is_ideographic(c: char) -> bool {
    matches!(c,
        '\u{3040}'..='\u{30ff}' | '\u{3400}'..='\u{4dbf}' | '\u{4e00}'..='\u{9fff}' | '\u{f900}'..='\u{faff}')
}

/// Words are whitespace-separated runs; scripts written without spaces
/// (Chinese, Japanese kana) count each character as one word.
pub fn count_words(text: &str) -> usize {
    let mut words = 0;
    let mut in_word = false;
    for c in text.chars() {
        if c.is_whitespace() {
            in_word = false;
        } else if is_ideographic(c) {
            words += 1;
            in_word = false;
        } else if !in_word {
            words += 1;
            in_word = true;
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words_in_spaced_and_unspaced_scripts() {
        assert_eq!(count_words("Hello, big  world!\n"), 3);
        assert_eq!(count_words("你好，世界"), 4 + 1);
        assert_eq!(count_words(""), 0);
    }
}
