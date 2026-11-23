#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod export;
mod simulation;
mod stats;

use std::time::Instant;

use eframe::egui;
use simulation::{Simulation, TokenizerKind};
use stats::Stats;

const EXAMPLES: &[(&str, &str)] = &[
    (
        "Русский",
        "Искусственный интеллект помогает нам исследовать мир, учиться новому и делиться идеями. Каждый токен появляется с заданной скоростью.",
    ),
    (
        "English",
        "Artificial intelligence helps us explore the world, learn new things, and share ideas. Each token appears at the selected speed.",
    ),
    (
        "Español",
        "La inteligencia artificial nos ayuda a explorar el mundo, aprender cosas nuevas y compartir ideas. Cada token aparece a la velocidad elegida.",
    ),
    (
        "Français",
        "L'intelligence artificielle nous aide à explorer le monde, à apprendre et à partager des idées. Chaque jeton apparaît à la vitesse choisie.",
    ),
    (
        "Deutsch",
        "Künstliche Intelligenz hilft uns, die Welt zu erkunden, Neues zu lernen und Ideen zu teilen. Jedes Token erscheint mit der gewählten Geschwindigkeit.",
    ),
    (
        "Português",
        "A inteligência artificial nos ajuda a explorar o mundo, aprender e compartilhar ideias. Cada token aparece na velocidade escolhida.",
    ),
    (
        "Українська",
        "Штучний інтелект допомагає нам досліджувати світ, вчитися та ділитися ідеями. Кожен токен з'являється із заданою швидкістю.",
    ),
    (
        "中文",
        "人工智能帮助我们探索世界、学习新知识并分享想法。每个词元都按照设定的速度出现。",
    ),
    (
        "日本語",
        "人工知能は、世界を探検し、新しいことを学び、アイデアを共有する助けになります。トークンは設定された速度で表示されます。",
    ),
    (
        "한국어",
        "인공지능은 우리가 세상을 탐험하고 새로운 것을 배우며 생각을 나누도록 돕습니다. 각 토큰은 설정한 속도로 나타납니다.",
    ),
    (
        "العربية",
        "يساعدنا الذكاء الاصطناعي على استكشاف العالم وتعلم أشياء جديدة ومشاركة الأفكار. يظهر كل رمز بالسرعة المحددة.",
    ),
    (
        "हिन्दी",
        "कृत्रिम बुद्धिमत्ता हमें दुनिया को समझने, नई बातें सीखने और विचार साझा करने में मदद करती है। हर टोकन चुनी हुई गति से दिखाई देता है।",
    ),
    (
        "Türkçe",
        "Yapay zekâ dünyayı keşfetmemize, yeni şeyler öğrenmemize ve fikirleri paylaşmamıza yardımcı olur. Her belirteç seçilen hızda görünür.",
    ),
    (
        "עברית",
        "בינה מלאכותית עוזרת לנו לחקור את העולם, ללמוד דברים חדשים ולשתף רעיונות. כל טוקן מופיע במהירות שנבחרה.",
    ),
];

const EXTRA_PARAGRAPHS: &[&str] = &[
    "Утром исследователи собираются вместе и обсуждают вопросы, на которые ещё нет ответов. Один человек изучает фотографии далёких планет, другой наблюдает за жизнью океана, а третий пишет программу для анализа данных. Они сравнивают результаты, проверяют предположения и делятся открытиями.\n\nТехнологии полезны тогда, когда помогают людям лучше понимать друг друга. Перевод текста, поиск информации и доступное образование открывают новые возможности для людей в разных странах. Но результаты работы любой системы важно внимательно проверять и учитывать контекст.\n\nВечером команда подводит итоги дня. Некоторые задачи оказались сложнее, чем ожидалось, зато появились новые идеи. Завтра работа продолжится: любопытство и терпение помогают постепенно находить ответы.",
    "In the morning, a group of researchers gathers to discuss questions that have no easy answers. One studies photographs of distant planets, another observes life in the ocean, and a third writes a program to explore data. They compare their results, test assumptions, and share what they discover.\n\nTechnology is most useful when it helps people understand one another. Translation, access to information, and education can create new opportunities for communities around the world. Yet the output of any system should be checked carefully, with attention to its context and limitations.\n\nLater, the team looks back on the day. Some problems proved harder than expected, but the discussion also inspired new ideas. Tomorrow the work will continue: curiosity, collaboration, and patience help turn small observations into meaningful discoveries.\n\nA language model produces text one token at a time, but tokens are not the same as words or letters. Common expressions may fit into a few tokens, while an unfamiliar name or a different writing system may require more. The visible pace therefore depends on both the tokenizer and the text.",
    "Por la mañana, un grupo de investigadores comparte preguntas e ideas. Unos observan el océano, otros estudian las estrellas y otros escriben programas para analizar datos. Juntos comparan los resultados y comprueban sus hipótesis.\n\nLa tecnología puede acercar a personas de distintos lugares, facilitar el aprendizaje y ayudar a descubrir nuevas perspectivas. Es importante revisar cada resultado y comprender su contexto antes de sacar conclusiones.",
    "Le matin, une équipe de recherche partage ses questions et ses idées. Certains observent l'océan, d'autres étudient les étoiles ou écrivent des programmes pour analyser des données. Ensemble, ils comparent les résultats et vérifient leurs hypothèses.\n\nLa technologie peut rapprocher les personnes, faciliter l'apprentissage et faire découvrir de nouvelles perspectives. Il reste important de vérifier les résultats et de comprendre leur contexte.",
    "Am Morgen tauscht ein Forschungsteam Fragen und Ideen aus. Einige beobachten den Ozean, andere untersuchen Sterne oder schreiben Programme zur Datenanalyse. Gemeinsam vergleichen sie Ergebnisse und überprüfen ihre Annahmen.\n\nTechnologie kann Menschen verbinden, das Lernen erleichtern und neue Perspektiven eröffnen. Ergebnisse sollten dennoch sorgfältig geprüft und in ihrem Zusammenhang betrachtet werden.",
    "Pela manhã, uma equipe de pesquisadores compartilha perguntas e ideias. Alguns observam o oceano, outros estudam as estrelas ou escrevem programas para analisar dados. Juntos, comparam os resultados e verificam suas hipóteses.\n\nA tecnologia pode aproximar pessoas, facilitar o aprendizado e abrir novas perspectivas. Ainda assim, é importante verificar os resultados e compreender seu contexto.",
    "Вранці дослідники обговорюють запитання та нові ідеї. Одні вивчають океан, інші спостерігають за зорями або пишуть програми для аналізу даних. Разом вони порівнюють результати та перевіряють припущення.\n\nТехнології можуть об'єднувати людей, полегшувати навчання й відкривати нові можливості. Водночас кожен результат варто ретельно перевіряти та розглядати в контексті.",
    "早晨，研究人员聚在一起讨论问题和想法。有人观察海洋，有人研究遥远的星星，还有人编写程序来分析数据。他们共同比较结果，检验假设，分享新的发现。\n\n技术可以帮助不同地方的人们交流，促进学习，并带来新的视角。不过，我们仍然需要仔细检查每个结果，理解它的背景和局限。",
    "朝、研究者たちは集まって疑問やアイデアを話し合います。海を観察する人もいれば、星を調べたり、データを分析するプログラムを書いたりする人もいます。みんなで結果を比べ、仮説を確かめ、新しい発見を共有します。\n\n技術は人々をつなぎ、学びを助け、新しい視点をもたらします。ただし、結果を注意深く確認し、その背景や限界を理解することも大切です。",
    "아침에 연구자들은 모여 질문과 아이디어를 나눕니다. 어떤 사람은 바다를 관찰하고, 다른 사람은 별을 연구하거나 데이터를 분석하는 프로그램을 작성합니다. 그들은 결과를 비교하고 가설을 확인하며 새로운 발견을 공유합니다.\n\n기술은 사람들을 연결하고 배움을 돕고 새로운 관점을 열어 줍니다. 하지만 결과를 주의 깊게 확인하고 그 맥락과 한계를 이해하는 것도 중요합니다.",
    "في الصباح يجتمع الباحثون لمناقشة الأسئلة والأفكار. يراقب بعضهم المحيط، ويدرس آخرون النجوم أو يكتبون برامج لتحليل البيانات. يقارن الفريق النتائج ويتحقق من الفرضيات ويشارك الاكتشافات الجديدة.\n\nيمكن للتكنولوجيا أن تقرب الناس وتساعدهم على التعلم واكتشاف وجهات نظر مختلفة. ومع ذلك من المهم مراجعة النتائج بعناية وفهم سياقها وحدودها.",
    "सुबह शोधकर्ता अपने सवालों और विचारों पर चर्चा करने के लिए इकट्ठा होते हैं। कोई समुद्र का अध्ययन करता है, कोई तारों को देखता है और कोई आँकड़ों का विश्लेषण करने के लिए कार्यक्रम लिखता है। वे परिणामों की तुलना करते हैं और अपनी धारणाओं की जाँच करते हैं।\n\nतकनीक लोगों को जोड़ सकती है, सीखने में मदद कर सकती है और नए दृष्टिकोण खोल सकती है। फिर भी हर परिणाम को ध्यान से जाँचना और उसके संदर्भ को समझना ज़रूरी है।",
    "Sabah araştırmacılar sorularını ve fikirlerini paylaşmak için bir araya gelir. Bazıları okyanusu inceler, bazıları yıldızları gözlemler veya verileri analiz etmek için program yazar. Sonuçları karşılaştırır ve varsayımlarını sınarlar.\n\nTeknoloji insanları birbirine yaklaştırabilir, öğrenmeyi kolaylaştırabilir ve yeni bakış açıları sunabilir. Yine de sonuçları dikkatle kontrol etmek ve bağlamlarını anlamak önemlidir.",
    "בבוקר החוקרים נפגשים כדי לדון בשאלות וברעיונות. חלקם מתבוננים באוקיינוס, אחרים חוקרים כוכבים או כותבים תוכנות לניתוח נתונים. יחד הם משווים תוצאות, בודקים השערות ומשתפים תגליות חדשות.\n\nטכנולוגיה יכולה לחבר בין אנשים, לסייע בלמידה ולפתוח נקודות מבט חדשות. עם זאת חשוב לבדוק את התוצאות בזהירות ולהבין את ההקשר והמגבלות שלהן.",
];

fn repeat_text(passage: &str) -> String {
    let repetitions = 16_000_usize.div_ceil(passage.chars().count().max(1));
    std::iter::repeat_n(passage, repetitions)
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn base_text(index: usize) -> String {
    format!("{}\n\n{}", EXAMPLES[index].1, EXTRA_PARAGRAPHS[index])
}

fn short_label(name: &str) -> String {
    let parts: Vec<_> = std::path::Path::new(name)
        .components()
        .rev()
        .take(2)
        .map(|part| part.as_os_str().to_string_lossy().into_owned())
        .collect();
    parts.into_iter().rev().collect::<Vec<_>>().join("/")
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Texts,
    Efficiency,
    Compare,
    Matrix,
    Simulator,
}

#[derive(Clone, Copy, PartialEq)]
enum Metric {
    CharsPerToken,
    BytesPerToken,
    TokensPerWord,
}

impl Metric {
    fn label(self) -> &'static str {
        match self {
            Self::CharsPerToken => "characters per token (higher = better compression)",
            Self::BytesPerToken => "UTF-8 bytes per token (higher = better compression)",
            Self::TokensPerWord => "tokens per word (lower = better compression)",
        }
    }

    fn value(self, stats: &Stats) -> f64 {
        match self {
            Self::CharsPerToken => stats.chars_per_token(),
            Self::BytesPerToken => stats.bytes_per_token(),
            Self::TokensPerWord => stats.tokens_per_word(),
        }
    }
}

const BAR_COLORS: [egui::Color32; 6] = [
    egui::Color32::from_rgb(70, 130, 200),
    egui::Color32::from_rgb(220, 140, 50),
    egui::Color32::from_rgb(80, 170, 100),
    egui::Color32::from_rgb(180, 90, 170),
    egui::Color32::from_rgb(200, 80, 80),
    egui::Color32::from_rgb(120, 120, 120),
];

const BEST_COLOR: egui::Color32 = egui::Color32::from_rgb(70, 170, 100);

// Thousands separated by thin spaces for readability.
fn group(value: usize) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push('\u{202f}');
        }
        out.push(c);
    }
    out
}

fn number_cell(ui: &mut egui::Ui, text: egui::RichText) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(text);
    });
}

fn tab_description(tab: Tab) -> &'static str {
    match tab {
        Tab::Texts => "Read and edit the text of each language. Statistics update immediately.",
        Tab::Efficiency => "How compactly one tokenizer encodes every language.",
        Tab::Compare => {
            "How tokenizers compare with each other, per language or across all of them."
        }
        Tab::Matrix => {
            "All languages and tokenizers at a glance; colour shows relative compression."
        }
        Tab::Simulator => {
            "Live output of every language and tokenizer at the same tokens per second."
        }
    }
}

fn apply_style(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        use egui::{FontId, TextStyle};
        style
            .text_styles
            .insert(TextStyle::Body, FontId::proportional(15.0));
        style
            .text_styles
            .insert(TextStyle::Button, FontId::proportional(15.0));
        style
            .text_styles
            .insert(TextStyle::Heading, FontId::proportional(22.0));
        style
            .text_styles
            .insert(TextStyle::Small, FontId::proportional(12.5));
        style.spacing.item_spacing = egui::vec2(10.0, 8.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style.spacing.interact_size.y = 26.0;
    });
}

struct App {
    tokenizers: Vec<TokenizerKind>,
    // stats[tokenizer][language] for the base passage of each example.
    stats: Vec<Vec<Stats>>,
    current: usize,
    tab: Tab,
    metric: Metric,
    view_language: usize,
    efficiency_tokenizer: usize,
    sort_efficiency: bool,
    include_builtin: bool,
    compare_language: usize,
    show_samples: bool,
    sims: Vec<Vec<Simulation>>,
    texts: Vec<String>,
    speed: f64,
    prepared: bool,
    error: Option<String>,
}

impl App {
    fn new() -> Result<Self, String> {
        let mut app = Self {
            tokenizers: Vec::new(),
            stats: Vec::new(),
            current: 0,
            tab: Tab::Texts,
            metric: Metric::CharsPerToken,
            view_language: 0,
            efficiency_tokenizer: 0,
            sort_efficiency: true,
            include_builtin: false,
            compare_language: EXAMPLES.len(),
            show_samples: false,
            sims: Vec::new(),
            texts: (0..EXAMPLES.len()).map(base_text).collect(),
            speed: 20.0,
            prepared: false,
            error: None,
        };
        app.add_tokenizer(TokenizerKind::openai("o200k_base")?)?;
        app.add_tokenizer(TokenizerKind::openai("cl100k_base")?)?;
        app.current = 0;
        app.invalidate();
        Ok(app)
    }

    fn load_tokenizers(&mut self) {
        let Some(paths) = rfd::FileDialog::new()
            .add_filter("Hugging Face tokenizer", &["json"])
            .pick_files()
        else {
            return;
        };
        let mut errors = Vec::new();
        for path in paths {
            let result = TokenizerKind::hugging_face(&path)
                .and_then(|tokenizer| self.add_tokenizer(tokenizer));
            if let Err(error) = result {
                errors.push(format!("{}: {error}", path.display()));
            }
        }
        self.invalidate();
        if !errors.is_empty() {
            self.error = Some(format!("Could not load tokenizer: {}", errors.join("; ")));
        }
    }

    fn export_report(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("HTML", &["html"])
            .set_file_name("tokenizer-statistics.html")
            .save_file()
        else {
            return;
        };
        let languages: Vec<&str> = EXAMPLES.iter().map(|(name, _)| *name).collect();
        let html = export::build_report(&self.labels(), &languages, &self.visible_stats());
        self.error = std::fs::write(&path, html)
            .err()
            .map(|error| format!("Could not save {}: {error}", path.display()));
    }

    fn tokenizer_bar(&mut self, ui: &mut egui::Ui) {
        let labels = self.all_labels();
        let picked = std::cell::Cell::new(None);
        let pick = |ui: &mut egui::Ui, index: usize, app: &Self| {
            if ui
                .selectable_label(app.current == index, &labels[index])
                .on_hover_text(app.tokenizers[index].name())
                .clicked()
            {
                picked.set(Some(index));
            }
        };
        let has_custom = self.tokenizers.iter().any(|t| !t.is_builtin());
        if has_custom {
            egui::CollapsingHeader::new("Built-in tokenizers")
                .id_salt("builtin_tokenizers")
                .show(ui, |ui| {
                    ui.checkbox(
                        &mut self.include_builtin,
                        "Include built-in tokenizers in comparisons and simulation",
                    );
                    ui.horizontal_wrapped(|ui| {
                        for index in
                            (0..self.tokenizers.len()).filter(|&i| self.tokenizers[i].is_builtin())
                        {
                            pick(ui, index, self);
                        }
                    });
                });
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("Tokenizer:");
            for index in 0..self.tokenizers.len() {
                if !has_custom || !self.tokenizers[index].is_builtin() {
                    pick(ui, index, self);
                }
            }
            if let Some(index) = picked.take()
                && self.current != index
            {
                self.current = index;
                self.invalidate();
            }
            if ui.button("Export HTML report…").clicked() {
                self.export_report();
            }
            if ui.button("Load tokenizer.json files…").clicked() {
                self.load_tokenizers();
            }
            if self.tokenizers.len() > 1 && ui.button("Remove selected").clicked() {
                self.tokenizers.remove(self.current);
                self.stats.remove(self.current);
                self.current = 0;
                self.efficiency_tokenizer = 0;
                self.invalidate();
            }
        });
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
    }

    fn refresh_stats(&mut self, language: usize) {
        for (tokenizer, stats) in self.tokenizers.iter().zip(&mut self.stats) {
            match Stats::measure(tokenizer, &self.texts[language]) {
                Ok(measured) => stats[language] = measured,
                Err(error) => self.error = Some(format!("Tokenization failed: {error}")),
            }
        }
    }

    fn add_tokenizer(&mut self, tokenizer: TokenizerKind) -> Result<(), String> {
        let stats = self
            .texts
            .iter()
            .map(|text| Stats::measure(&tokenizer, text))
            .collect::<Result<Vec<_>, _>>()?;
        self.tokenizers.push(tokenizer);
        self.stats.push(stats);
        self.current = self.tokenizers.len() - 1;
        Ok(())
    }

    fn all_labels(&self) -> Vec<String> {
        self.tokenizers
            .iter()
            .map(|tokenizer| short_label(tokenizer.name()))
            .collect()
    }

    // Built-in tokenizers are left out of comparisons once custom ones are loaded,
    // unless explicitly included.
    fn visible(&self) -> Vec<usize> {
        let has_custom = self.tokenizers.iter().any(|t| !t.is_builtin());
        (0..self.tokenizers.len())
            .filter(|&i| self.include_builtin || !has_custom || !self.tokenizers[i].is_builtin())
            .collect()
    }

    fn labels(&self) -> Vec<String> {
        let labels = self.all_labels();
        self.visible()
            .into_iter()
            .map(|i| labels[i].clone())
            .collect()
    }

    fn visible_stats(&self) -> Vec<&Vec<Stats>> {
        self.visible().into_iter().map(|i| &self.stats[i]).collect()
    }

    fn texts_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for (index, (name, _)) in EXAMPLES.iter().enumerate() {
                ui.selectable_value(&mut self.view_language, index, *name);
            }
        });
        ui.horizontal(|ui| {
            if ui.button("◀ Previous").clicked() {
                self.view_language = (self.view_language + EXAMPLES.len() - 1) % EXAMPLES.len();
            }
            if ui.button("Next ▶").clicked() {
                self.view_language = (self.view_language + 1) % EXAMPLES.len();
            }
        });
        let language = self.view_language;
        ui.heading(EXAMPLES[language].0);
        let changed = ui
            .add(
                egui::TextEdit::multiline(&mut self.texts[language])
                    .desired_rows(8)
                    .desired_width(f32::INFINITY),
            )
            .changed();
        if ui.button("Restore example").clicked() {
            self.texts[language] = base_text(language);
            self.refresh_stats(language);
            self.invalidate();
        } else if changed {
            self.refresh_stats(language);
            self.invalidate();
        }
        ui.add_space(8.0);
        let labels = self.labels();
        let visible_stats = self.visible_stats();
        let first = &visible_stats[0][language];
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.strong("Length:");
                ui.label(format!("{} characters (no whitespace)", group(first.chars)));
                ui.label("·");
                ui.label(format!("{} UTF-8 bytes", group(first.bytes)));
                ui.label("·");
                ui.strong("Linguistic length:");
                ui.label(format!("{} words", group(first.words)));
            });
        });
        ui.add_space(4.0);
        let best = visible_stats
            .iter()
            .map(|s| s[language].chars_per_token())
            .fold(f64::MIN, f64::max);
        egui::Grid::new("text_stats")
            .striped(true)
            .num_columns(5)
            .spacing([24.0, 6.0])
            .show(ui, |ui| {
                ui.strong("Tokenizer");
                for header in ["Tokens", "Chars/token", "Bytes/token", "Tokens/word"] {
                    number_cell(ui, egui::RichText::new(header).strong());
                }
                ui.end_row();
                for (label, stats) in labels.iter().zip(&visible_stats) {
                    let stats = &stats[language];
                    let is_best = (stats.chars_per_token() - best).abs() < 1e-9;
                    let mark = |text: String| {
                        let text = egui::RichText::new(text);
                        if is_best {
                            text.strong().color(BEST_COLOR)
                        } else {
                            text
                        }
                    };
                    ui.label(mark(label.clone()));
                    number_cell(ui, mark(group(stats.tokens)));
                    number_cell(ui, mark(format!("{:.2}", stats.chars_per_token())));
                    number_cell(ui, mark(format!("{:.2}", stats.bytes_per_token())));
                    number_cell(ui, mark(format!("{:.2}", stats.tokens_per_word())));
                    ui.end_row();
                }
            });
        ui.add_space(4.0);
        ui.small("Green marks the tokenizer with the best compression (most characters per token). Statistics are computed for the text above, not the repeated simulator text.");
    }

    fn metric_selector(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Metric")
            .selected_text(self.metric.label())
            .show_ui(ui, |ui| {
                for metric in [
                    Metric::CharsPerToken,
                    Metric::BytesPerToken,
                    Metric::TokensPerWord,
                ] {
                    ui.selectable_value(&mut self.metric, metric, metric.label());
                }
            });
    }

    fn efficiency_tab(&mut self, ui: &mut egui::Ui) {
        let labels = self.labels();
        self.efficiency_tokenizer = self.efficiency_tokenizer.min(labels.len() - 1);
        ui.horizontal_wrapped(|ui| {
            egui::ComboBox::from_label("Tokenizer")
                .selected_text(&labels[self.efficiency_tokenizer])
                .show_ui(ui, |ui| {
                    for (index, label) in labels.iter().enumerate() {
                        ui.selectable_value(&mut self.efficiency_tokenizer, index, label);
                    }
                });
            self.metric_selector(ui);
            ui.checkbox(&mut self.sort_efficiency, "Sort by efficiency");
        });
        ui.add_space(8.0);
        let stats = self.visible_stats()[self.efficiency_tokenizer];
        let values: Vec<f64> = stats.iter().map(|s| self.metric.value(s)).collect();
        let max = values.iter().cloned().fold(f64::MIN_POSITIVE, f64::max);
        let higher_is_better = self.metric != Metric::TokensPerWord;
        let mut order: Vec<usize> = (0..EXAMPLES.len()).collect();
        if self.sort_efficiency {
            order.sort_by(|&a, &b| {
                let ordering = values[a].total_cmp(&values[b]);
                if higher_is_better {
                    ordering.reverse()
                } else {
                    ordering
                }
            });
        }
        let best = order.first().copied();
        for index in order {
            let is_best = self.sort_efficiency && Some(index) == best;
            ui.horizontal(|ui| {
                ui.add_sized([100.0, 22.0], egui::Label::new(EXAMPLES[index].0));
                ui.add(
                    egui::ProgressBar::new((values[index] / max) as f32)
                        .desired_width(ui.available_width())
                        .desired_height(22.0)
                        .fill(if is_best { BEST_COLOR } else { BAR_COLORS[0] })
                        .text(format!("{:.2}", values[index])),
                );
            });
        }
        ui.add_space(6.0);
        ui.small("Bars are scaled to the best language for this tokenizer.");
    }

    fn compare_tab(&mut self, ui: &mut egui::Ui) {
        let labels = self.labels();
        ui.horizontal_wrapped(|ui| {
            let selected = EXAMPLES
                .get(self.compare_language)
                .map_or("All languages", |e| e.0);
            egui::ComboBox::from_label("Language")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.compare_language,
                        EXAMPLES.len(),
                        "All languages",
                    );
                    for (index, (name, _)) in EXAMPLES.iter().enumerate() {
                        ui.selectable_value(&mut self.compare_language, index, *name);
                    }
                });
            self.metric_selector(ui);
        });
        ui.horizontal_wrapped(|ui| {
            for (index, label) in labels.iter().enumerate() {
                ui.colored_label(BAR_COLORS[index % BAR_COLORS.len()], format!("■ {label}"));
            }
        });
        ui.add_space(8.0);
        let languages: Vec<usize> = match self.compare_language {
            index if index < EXAMPLES.len() => vec![index],
            _ => (0..EXAMPLES.len()).collect(),
        };
        let visible_stats = self.visible_stats();
        let max = languages
            .iter()
            .flat_map(|&language| visible_stats.iter().map(move |s| (language, s)))
            .map(|(language, s)| self.metric.value(&s[language]))
            .fold(f64::MIN_POSITIVE, f64::max);
        for language in languages {
            ui.label(egui::RichText::new(EXAMPLES[language].0).strong());
            for (index, stats) in visible_stats.iter().enumerate() {
                let value = self.metric.value(&stats[language]);
                let base = self.metric.value(&visible_stats[0][language]);
                let versus = if base > 0.0 {
                    value / base * 100.0
                } else {
                    0.0
                };
                ui.add(
                    egui::ProgressBar::new((value / max) as f32)
                        .fill(BAR_COLORS[index % BAR_COLORS.len()])
                        .text(format!("{value:.2} ({versus:.0}% of {})", labels[0])),
                );
            }
            ui.add_space(6.0);
        }
    }

    fn matrix_tab(&mut self, ui: &mut egui::Ui) {
        let labels = self.labels();
        ui.horizontal_wrapped(|ui| {
            self.metric_selector(ui);
            ui.checkbox(&mut self.show_samples, "Show first tokens of each text");
        });
        ui.small("Cell colour: green = better compression, red = worse (relative to all cells).");
        let visible_stats = self.visible_stats();
        let values: Vec<f64> = visible_stats
            .iter()
            .flat_map(|s| s.iter())
            .map(|s| self.metric.value(s))
            .collect();
        let min = values.iter().cloned().fold(f64::MAX, f64::min);
        let max = values.iter().cloned().fold(f64::MIN, f64::max);
        let better_is_high = self.metric != Metric::TokensPerWord;
        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("matrix")
                .striped(false)
                .spacing([6.0, 6.0])
                .show(ui, |ui| {
                    ui.strong("Language");
                    for label in &labels {
                        ui.strong(label);
                    }
                    ui.end_row();
                    for (language, (name, _)) in EXAMPLES.iter().enumerate() {
                        ui.strong(*name);
                        for stats in &visible_stats {
                            let stats = &stats[language];
                            let value = self.metric.value(stats);
                            let mut t = if max > min {
                                ((value - min) / (max - min)) as f32
                            } else {
                                0.5
                            };
                            if !better_is_high {
                                t = 1.0 - t;
                            }
                            let color = egui::Color32::from_rgb(
                                (230.0 - 100.0 * t) as u8,
                                (120.0 + 90.0 * t) as u8,
                                110,
                            );
                            egui::Frame::new()
                                .fill(color)
                                .inner_margin(4.0)
                                .show(ui, |ui| {
                                    ui.set_min_width(110.0);
                                    ui.colored_label(
                                        egui::Color32::BLACK,
                                        format!("{:.2}  ({} tok.)", value, stats.tokens),
                                    );
                                    if self.show_samples {
                                        ui.colored_label(egui::Color32::BLACK, &stats.sample);
                                    }
                                });
                        }
                        ui.end_row();
                    }
                });
        });
    }

    fn invalidate(&mut self) {
        self.sims = self
            .tokenizers
            .iter()
            .map(|_| (0..EXAMPLES.len()).map(|_| Simulation::default()).collect())
            .collect();
        self.prepared = false;
        self.error = None;
    }

    fn all_sims(&mut self) -> impl Iterator<Item = &mut Simulation> {
        self.sims.iter_mut().flatten()
    }

    fn running(&self) -> bool {
        self.sims.iter().flatten().any(|sim| sim.running)
    }

    fn prepare(&mut self) {
        let texts: Vec<String> = self.texts.iter().map(|text| repeat_text(text)).collect();
        for (tokenizer, sims) in self.tokenizers.iter().zip(&mut self.sims) {
            for (sim, text) in sims.iter_mut().zip(&texts) {
                if let Err(error) = sim.prepare(tokenizer, text) {
                    self.invalidate();
                    self.error = Some(format!("Tokenization failed: {error}"));
                    return;
                }
            }
        }
        self.prepared = true;
        self.error = None;
    }

    fn simulator_tab(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Slider::new(&mut self.speed, 1.0..=1500.0)
                .integer()
                .text("tokens/s"),
        );
        ui.horizontal_wrapped(|ui| {
            if ui.button("Count tokens").clicked() {
                self.prepare();
            }
            if self.running() {
                if ui.button("Pause").clicked() {
                    self.all_sims().for_each(Simulation::pause);
                }
            } else {
                let partial = self
                    .sims
                    .iter()
                    .flatten()
                    .any(|sim| sim.shown > 0 && sim.shown < sim.ids.len());
                if ui
                    .button(if partial { "Resume" } else { "Start" })
                    .clicked()
                {
                    if !self.prepared {
                        self.prepare();
                    }
                    if self.prepared {
                        let now = Instant::now();
                        self.all_sims().for_each(|sim| sim.start(now));
                    }
                }
            }
            if ui.button("Restart").clicked() && self.prepared {
                let now = Instant::now();
                self.all_sims().for_each(|sim| {
                    sim.shown = sim.ids.len();
                    sim.start(now);
                });
            }
        });
        if !self.prepared {
            ui.label("Count tokens or start the simulation. The texts come from the Texts tab and are repeated to keep playback visible at high speeds.");
        }
        ui.small("This is a simulation, not model generation. Every cell runs at the same speed; decoded text may differ from the input and only the latest characters are shown.");
        let labels = self.labels();
        let visible_sims: Vec<&Vec<Simulation>> =
            self.visible().into_iter().map(|i| &self.sims[i]).collect();
        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("sim_matrix")
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    ui.strong("Language");
                    for label in &labels {
                        ui.strong(label);
                    }
                    ui.end_row();
                    for (language, (name, _)) in EXAMPLES.iter().enumerate() {
                        ui.strong(*name);
                        for sims in &visible_sims {
                            let sim = &sims[language];
                            ui.vertical(|ui| {
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(CELL_WIDTH, CELL_HEIGHT),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect(
                                    rect,
                                    4.0,
                                    ui.visuals().extreme_bg_color,
                                    ui.visuals().widgets.noninteractive.bg_stroke,
                                    egui::StrokeKind::Inside,
                                );
                                let mut cell =
                                    ui.new_child(egui::UiBuilder::new().max_rect(rect.shrink(6.0)));
                                cell.set_clip_rect(rect.shrink(2.0).intersect(ui.clip_rect()));
                                cell.label(highlighted(
                                    page(&sim.output, PAGE_CHARS),
                                    cell.visuals().text_color(),
                                    sim.running,
                                ));
                                if self.prepared && !sim.ids.is_empty() {
                                    let total = sim.ids.len();
                                    ui.add_sized(
                                        [CELL_WIDTH, 14.0],
                                        egui::ProgressBar::new(sim.shown as f32 / total as f32)
                                            .text(format!("{} / {} tokens", sim.shown, total)),
                                    );
                                    ui.small(format!(
                                        "Total ≈ {:.1} s, left ≈ {:.1} s",
                                        total as f64 / self.speed,
                                        (total - sim.shown) as f64 / self.speed
                                    ));
                                }
                            });
                        }
                        ui.end_row();
                    }
                });
        });
    }
}

const CELL_WIDTH: f32 = 320.0;
const CELL_HEIGHT: f32 = 130.0;
const PAGE_CHARS: usize = 200;
const FRESH_CHARS: usize = 12;

// The newest characters are drawn in an accent colour so growth is easy to follow.
fn highlighted(text: &str, color: egui::Color32, running: bool) -> egui::text::LayoutJob {
    let split = text
        .char_indices()
        .rev()
        .nth(FRESH_CHARS.saturating_sub(1))
        .map_or(0, |(i, _)| i);
    let font_id = egui::FontId::proportional(13.0);
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &text[..split],
        0.0,
        egui::TextFormat::simple(font_id.clone(), color),
    );
    let accent = egui::Color32::from_rgb(255, 170, 40);
    job.append(
        &text[split..],
        0.0,
        egui::TextFormat::simple(font_id.clone(), accent),
    );
    if running {
        job.append("▌", 0.0, egui::TextFormat::simple(font_id, accent));
    }
    job
}

// Shows the text from the start of the current "page" so that earlier lines
// stay in place while new text is appended; the cell clears when a page fills.
fn page(text: &str, page_chars: usize) -> &str {
    let skip = text.chars().count() / page_chars * page_chars;
    text.char_indices()
        .nth(skip)
        .map_or("", |(i, _)| &text[i..])
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let speed = self.speed;
        let mut failure = None;
        for (tokenizer, sims) in self.tokenizers.iter().zip(&mut self.sims) {
            for sim in sims {
                if let Err(error) = sim.advance(now, speed, tokenizer) {
                    failure = Some(error);
                }
            }
        }
        if let Some(error) = failure {
            self.all_sims().for_each(Simulation::pause);
            self.error = Some(format!("Decoding failed: {error}"));
        }
        egui::CentralPanel::default().show(ui, |ui| {
            self.tokenizer_bar(ui);
            ui.separator();
            ui.horizontal(|ui| {
                for (tab, name) in [
                    (Tab::Texts, "Texts"),
                    (Tab::Efficiency, "Compression efficiency"),
                    (Tab::Compare, "Tokenizer comparison"),
                    (Tab::Matrix, "Matrix"),
                    (Tab::Simulator, "Simulator"),
                ] {
                    ui.selectable_value(&mut self.tab, tab, egui::RichText::new(name).size(16.0));
                }
            });
            ui.separator();
            ui.label(egui::RichText::new(tab_description(self.tab)).weak());
            match self.tab {
                Tab::Simulator => self.simulator_tab(ui),
                tab => {
                    egui::ScrollArea::vertical()
                        .id_salt("tab_scroll")
                        .show(ui, |ui| match tab {
                            Tab::Texts => self.texts_tab(ui),
                            Tab::Efficiency => self.efficiency_tab(ui),
                            Tab::Compare => self.compare_tab(ui),
                            _ => self.matrix_tab(ui),
                        });
                }
            }
        });
        if let Some(sim) = self.sims.iter().flatten().find(|sim| sim.running) {
            ui.ctx().request_repaint_after(sim.next_repaint(self.speed));
        }
    }
}

fn main() -> eframe::Result {
    let result = run();
    #[cfg(all(windows, not(debug_assertions)))]
    if let Err(error) = &result {
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Tokens per second")
            .set_description(format!("Could not start the application: {error}"))
            .show();
    }
    result
}

fn run() -> eframe::Result {
    let app = App::new().map_err(|error| {
        eprintln!("Could not initialize tokenizer: {error}");
        eframe::Error::AppCreation(Box::new(std::io::Error::other(error)))
    })?;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 800.0])
            .with_min_inner_size([480.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tokens per second",
        options,
        Box::new(move |cc| {
            apply_style(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_text(index: usize) -> String {
        repeat_text(&base_text(index))
    }

    #[test]
    fn examples_are_long_enough_at_maximum_speed() {
        let tokenizer = TokenizerKind::openai("o200k_base").unwrap();
        assert_eq!(EXAMPLES.len(), EXTRA_PARAGRAPHS.len());
        for (index, (language, _)) in EXAMPLES.iter().enumerate() {
            let text = example_text(index);
            assert!(text.chars().count() >= 16_000, "{language}");
            assert!(
                tokenizer.encode(&text).unwrap().len() >= 1_500,
                "{language}"
            );
        }
    }

    #[test]
    fn maximum_speed_outputs_1500_tokens_in_one_second() {
        let tokenizer = TokenizerKind::openai("o200k_base").unwrap();
        let mut simulation = Simulation::default();
        simulation.prepare(&tokenizer, &example_text(1)).unwrap();
        let now = Instant::now();
        simulation.start(now);
        simulation
            .advance(now + std::time::Duration::from_secs(1), 1500.0, &tokenizer)
            .unwrap();
        assert_eq!(simulation.shown, 1500);
        assert!(simulation.running);
    }
}
