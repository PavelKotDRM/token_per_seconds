use std::time::{Duration, Instant};

use tiktoken_rs::CoreBPE;
use tokenizers::Tokenizer;

pub enum TokenizerKind {
    OpenAi {
        name: &'static str,
        bpe: CoreBPE,
    },
    HuggingFace {
        name: String,
        tokenizer: Box<Tokenizer>,
    },
}

impl TokenizerKind {
    pub fn openai(name: &'static str) -> Result<Self, String> {
        let bpe = match name {
            "o200k_base" => tiktoken_rs::o200k_base(),
            "cl100k_base" => tiktoken_rs::cl100k_base(),
            _ => return Err(format!("Unknown tokenizer: {name}")),
        }
        .map_err(|error| error.to_string())?;
        Ok(Self::OpenAi { name, bpe })
    }

    pub fn hugging_face(path: &std::path::Path) -> Result<Self, String> {
        let mut tokenizer = Tokenizer::from_file(path).map_err(|error| error.to_string())?;
        tokenizer
            .with_truncation(None)
            .map_err(|error| error.to_string())?;
        tokenizer.with_padding(None);
        Ok(Self::HuggingFace {
            name: path.display().to_string(),
            tokenizer: Box::new(tokenizer),
        })
    }

    pub fn is_builtin(&self) -> bool {
        matches!(self, Self::OpenAi { .. })
    }

    pub fn name(&self) -> &str {
        match self {
            Self::OpenAi { name, .. } => name,
            Self::HuggingFace { name, .. } => name,
        }
    }

    pub fn encode(&self, text: &str) -> Result<Vec<u32>, String> {
        match self {
            Self::OpenAi { bpe, .. } => Ok(bpe.encode_ordinary(text)),
            Self::HuggingFace { tokenizer, .. } => tokenizer
                .encode(text, false)
                .map(|encoding| encoding.get_ids().to_vec())
                .map_err(|error| error.to_string()),
        }
    }

    pub fn decode(&self, ids: &[u32]) -> Result<String, String> {
        match self {
            Self::OpenAi { bpe, .. } => {
                let bytes = bpe.decode_bytes(ids).map_err(|error| error.to_string())?;
                // A token can end in the middle of a UTF-8 character.
                Ok(String::from_utf8_lossy(&bytes).into_owned())
            }
            Self::HuggingFace { tokenizer, .. } => tokenizer
                .decode(ids, false)
                .map_err(|error| error.to_string()),
        }
    }
}

#[derive(Default)]
pub struct Simulation {
    pub ids: Vec<u32>,
    pub shown: usize,
    pub output: String,
    pub running: bool,
    credit: f64,
    last_tick: Option<Instant>,
}

impl Simulation {
    pub fn prepare(&mut self, tokenizer: &TokenizerKind, text: &str) -> Result<(), String> {
        let ids = tokenizer.encode(text)?;
        *self = Self {
            ids,
            ..Self::default()
        };
        Ok(())
    }

    pub fn start(&mut self, now: Instant) {
        if self.shown == self.ids.len() {
            self.shown = 0;
            self.output.clear();
            self.credit = 0.0;
        }
        self.running = !self.ids.is_empty();
        self.last_tick = self.running.then_some(now);
    }

    pub fn pause(&mut self) {
        self.running = false;
        self.last_tick = None;
    }

    pub fn advance(
        &mut self,
        now: Instant,
        speed: f64,
        tokenizer: &TokenizerKind,
    ) -> Result<(), String> {
        if !self.running {
            return Ok(());
        }
        let elapsed = now.saturating_duration_since(self.last_tick.unwrap_or(now));
        self.last_tick = Some(now);
        self.credit += elapsed.as_secs_f64() * speed;
        let count = (self.credit.floor() as usize).min(self.ids.len() - self.shown);
        if count > 0 {
            let next = self.shown + count;
            let output = tokenizer.decode(&self.ids[..next])?;
            self.shown = next;
            self.output = output;
            self.credit -= count as f64;
        }
        if self.shown == self.ids.len() {
            self.pause();
            self.credit = 0.0;
        }
        Ok(())
    }

    pub fn next_repaint(&self, speed: f64) -> Duration {
        Duration::from_secs_f64(((1.0 - self.credit) / speed).clamp(0.001, 0.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizers::models::wordlevel::WordLevel;
    use tokenizers::pre_tokenizers::whitespace::Whitespace;
    use tokenizers::utils::padding::{PaddingParams, PaddingStrategy};
    use tokenizers::utils::truncation::TruncationParams;

    #[test]
    fn openai_round_trips_multilingual_bytes() {
        let tokenizer = TokenizerKind::openai("o200k_base").unwrap();
        for text in ["Привет, мир!", "你好，世界！", "🙂"] {
            let ids = tokenizer.encode(text).unwrap();
            assert!(!ids.is_empty());
            assert_eq!(tokenizer.decode(&ids).unwrap(), text);
        }
    }

    #[test]
    fn custom_tokenizer_uses_real_ids_and_decoder() {
        let model = WordLevel::builder()
            .vocab(
                [
                    ("hello".into(), 0),
                    ("world".into(), 1),
                    ("[UNK]".into(), 2),
                ]
                .into_iter()
                .collect(),
            )
            .unk_token("[UNK]".into())
            .build()
            .unwrap();
        let mut inner = Tokenizer::new(model);
        inner.with_pre_tokenizer(Some(Whitespace));
        inner
            .with_truncation(Some(TruncationParams {
                max_length: 1,
                ..Default::default()
            }))
            .unwrap();
        inner.with_padding(Some(PaddingParams {
            strategy: PaddingStrategy::Fixed(8),
            ..Default::default()
        }));
        let path = std::env::temp_dir().join(format!(
            "token-per-seconds-test-{}.json",
            std::process::id()
        ));
        inner.save(&path, false).unwrap();
        let tokenizer = TokenizerKind::hugging_face(&path).unwrap();
        std::fs::remove_file(path).unwrap();
        let ids = tokenizer.encode("hello world").unwrap();
        assert_eq!(ids, vec![0, 1]);
        assert_eq!(tokenizer.decode(&ids).unwrap(), "hello world");
    }

    #[test]
    fn timing_pause_resume_speed_change_and_restart() {
        let tokenizer = TokenizerKind::openai("cl100k_base").unwrap();
        let mut simulation = Simulation::default();
        simulation
            .prepare(&tokenizer, "one two three four five six")
            .unwrap();
        let now = Instant::now();
        simulation.start(now);
        simulation
            .advance(now + Duration::from_millis(250), 2.0, &tokenizer)
            .unwrap();
        assert_eq!(simulation.shown, 0);
        simulation
            .advance(now + Duration::from_millis(500), 2.0, &tokenizer)
            .unwrap();
        assert_eq!(simulation.shown, 1);
        simulation.pause();
        simulation.start(now + Duration::from_secs(10));
        simulation
            .advance(now + Duration::from_millis(10_500), 4.0, &tokenizer)
            .unwrap();
        assert_eq!(simulation.shown, 3);
        simulation
            .advance(now + Duration::from_secs(100), 4.0, &tokenizer)
            .unwrap();
        assert_eq!(simulation.shown, simulation.ids.len());
        assert!(!simulation.running);
        simulation.start(now + Duration::from_secs(101));
        assert_eq!(simulation.shown, 0);
        assert!(simulation.output.is_empty());
    }
}
