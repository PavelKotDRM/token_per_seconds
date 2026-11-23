#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod simulation;

use std::time::Instant;

use eframe::egui;
use simulation::{Simulation, TokenizerKind};

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

fn example_text(index: usize) -> String {
    let passage = format!("{}\n\n{}", EXAMPLES[index].1, EXTRA_PARAGRAPHS[index]);
    let repetitions = 16_000_usize.div_ceil(passage.chars().count());
    std::iter::repeat_n(passage.as_str(), repetitions)
        .collect::<Vec<_>>()
        .join("\n\n")
}

struct App {
    tokenizer: TokenizerKind,
    simulation: Simulation,
    text: String,
    example: usize,
    speed: f64,
    prepared: bool,
    error: Option<String>,
}

impl App {
    fn new() -> Result<Self, String> {
        Ok(Self {
            tokenizer: TokenizerKind::openai("o200k_base")?,
            simulation: Simulation::default(),
            text: example_text(1),
            example: 1,
            speed: 20.0,
            prepared: false,
            error: None,
        })
    }

    fn invalidate(&mut self) {
        self.simulation = Simulation::default();
        self.prepared = false;
        self.error = None;
    }

    fn select_tokenizer(&mut self, result: Result<TokenizerKind, String>) {
        match result {
            Ok(tokenizer) => {
                self.tokenizer = tokenizer;
                self.invalidate();
            }
            Err(error) => self.error = Some(format!("Could not load tokenizer: {error}")),
        }
    }

    fn prepare(&mut self) {
        match self.simulation.prepare(&self.tokenizer, &self.text) {
            Ok(()) => {
                self.prepared = true;
                self.error = None;
            }
            Err(error) => {
                self.invalidate();
                self.error = Some(format!("Tokenization failed: {error}"));
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        if let Err(error) = self.simulation.advance(now, self.speed, &self.tokenizer) {
            self.simulation.pause();
            self.error = Some(format!("Decoding failed: {error}"));
        }
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Token generation simulator");
                ui.label("Compare how tokenizers split text and how output appears at the selected speed.");
                ui.add_space(12.0);
                ui.heading("1. Tokenizer");
                ui.horizontal_wrapped(|ui| {
                    if ui.button("OpenAI o200k_base").clicked() {
                        self.select_tokenizer(TokenizerKind::openai("o200k_base"));
                    }
                    if ui.button("OpenAI cl100k_base").clicked() {
                        self.select_tokenizer(TokenizerKind::openai("cl100k_base"));
                    }
                    if ui.button("Open tokenizer.json…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Hugging Face tokenizer", &["json"])
                            .pick_file()
                        {
                            self.select_tokenizer(TokenizerKind::hugging_face(&path));
                        }
                    }
                });
                ui.label(format!("Selected: {}", self.tokenizer.name()));
                ui.add_space(12.0);
                ui.heading("2. Text");
                ui.horizontal_wrapped(|ui| {
                    ui.label("Example:");
                    let old = self.example;
                    egui::ComboBox::from_id_salt("language")
                        .selected_text(EXAMPLES[self.example].0)
                        .show_ui(ui, |ui| {
                            for (index, (name, _)) in EXAMPLES.iter().enumerate() {
                                ui.selectable_value(&mut self.example, index, *name);
                            }
                        });
                    if old != self.example || ui.button("Restore example").clicked() {
                        self.text = example_text(self.example);
                        self.invalidate();
                    }
                });
                let changed = egui::ScrollArea::vertical()
                    .id_salt("input_text")
                    .max_height(140.0)
                    .show(ui, |ui| {
                        ui.add(egui::TextEdit::multiline(&mut self.text).desired_rows(5).desired_width(f32::INFINITY)).changed()
                    }).inner;
                if changed {
                    self.invalidate();
                }
                ui.small("Long examples repeat their passages. You can enter your own text in any language.");
                ui.add_space(12.0);
                ui.heading("3. Speed and playback");
                ui.add(egui::Slider::new(&mut self.speed, 1.0..=1500.0).integer().text("tokens/s"));
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Count tokens").clicked() {
                        self.prepare();
                    }
                    if self.simulation.running {
                        if ui.button("Pause").clicked() {
                            self.simulation.pause();
                        }
                    } else if ui.button(if self.simulation.shown > 0 && self.simulation.shown < self.simulation.ids.len() { "Resume" } else { "Start" }).clicked() {
                        if !self.prepared {
                            self.prepare();
                        }
                        if self.prepared {
                            self.simulation.start(Instant::now());
                        }
                    }
                    if ui.button("Restart").clicked() && self.prepared {
                        self.simulation.shown = self.simulation.ids.len();
                        self.simulation.start(Instant::now());
                    }
                });
                if let Some(error) = &self.error {
                    ui.colored_label(egui::Color32::LIGHT_RED, error);
                }
                if self.prepared {
                    let total = self.simulation.ids.len();
                    ui.label(format!("Displayed: {} / {} tokens", self.simulation.shown, total));
                    if total > 0 {
                        ui.add(egui::ProgressBar::new(self.simulation.shown as f32 / total as f32).show_percentage());
                        ui.label(format!("Estimated total time at current speed: {:.1} s", total as f64 / self.speed));
                    } else {
                        ui.label("The text contains no tokens.");
                    }
                } else {
                    ui.label("Count tokens or start the simulation.");
                }
                ui.add_space(12.0);
                ui.heading("Output");
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_height(120.0);
                    ui.set_width(ui.available_width());
                    ui.label(&self.simulation.output);
                });
                ui.small("This is a simulation, not model generation. Decoded text may differ from the input.");
            });
        });
        if self.simulation.running {
            ui.ctx()
                .request_repaint_after(self.simulation.next_repaint(self.speed));
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
        Box::new(move |_cc| Ok(Box::new(app))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
