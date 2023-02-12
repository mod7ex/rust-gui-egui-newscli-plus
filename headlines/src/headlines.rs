use news_api::NewsAPI;
use confy;
use serde::{Serialize, Deserialize};
use eframe::{
    Frame,
    egui::{
        self,
        Context,
        FontData,
        FontDefinitions,
        FontFamily,
        TextStyle,
        FontId,
        Ui,
        Layout,
        Color32,
        Hyperlink,
        Align,
        Separator,
        Button, TopBottomPanel, menu::bar, RichText, Label, Window
    }
};

const APP_NAME: &str = "headlines";
pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

#[derive(Serialize, Deserialize, Default)]
pub struct HeadlinesConfig {
    pub dark_mode: bool,
    pub api_key: String,

}

pub struct NewsCardData {
    pub title: String,
    pub description: String,
    pub url: String,
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
    pub is_api_key_initialized: bool,
}

impl Headlines {
    pub fn new() -> Self {
        let config: HeadlinesConfig = confy::load(APP_NAME, None).unwrap_or_default();

        Headlines {
            is_api_key_initialized: !config.api_key.is_empty(),
            articles: vec![],
            config
        }
    }

    pub fn fetch_news(&mut self) {
        let api_key = &self.config.api_key;

        if let Ok(response) = NewsAPI::new(api_key).fetch() {
            for a in response.articles() {
                self.articles.push(NewsCardData {
                    title: a.title().to_owned(),
                    url: a.url().to_owned(),
                    description: a.description().map(|s| s.to_string()).unwrap_or("...".to_string())
                });
            }
        }
    }

    pub fn configure_fonts(&self, ctx: &Context) {
        use FontFamily::Proportional;

        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert(
            "MesloLGS".to_owned(),
            FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf"))
        );

        font_def.families.get_mut(&Proportional).unwrap()
            .insert(0, "MesloLGS".to_owned());

        ctx.set_fonts(font_def);

        use TextStyle::*;
    
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(35.0, Proportional)),
            (Body, FontId::new(20.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);
    }

    pub fn render_news(&self, ui: &mut Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            
            // title
            let title = format!("▶ {}", a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }
            
            ui.add_space(PADDING);

            // description
            if ui.add(Button::new(&a.description)).clicked() {
                println!("clicked --------- ")
            };

            // render hyperlinks
            ui.style_mut().visuals.hyperlink_color = if self.config.dark_mode {
                CYAN
            } else {
                RED
            };
            
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more ...", &a.url))
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub fn render_top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            bar(ui, |ui| {
                // logo
                ui.with_layout(
                    Layout::left_to_right(Align::LEFT), |ui| {
                        ui.add(Label::new(RichText::new("📓").heading()))
                    }
                );

                // controls
                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    if ui.add(Button::new(RichText::new("❌").heading())).clicked() {
                        frame.close();
                    };
                    if ui.add(Button::new(RichText::new("🔄").heading())).clicked() {
                        todo!();
                    };
                    if ui.add(Button::new(RichText::new(
                        if self.config.dark_mode {
                            "🌞"
                        } else {
                            "🌙"
                        }
                    ).heading())).clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    };
                });
            });
            ui.add_space(10.);
        });
    }

    pub fn render_config(&mut self, ctx: &Context, frame: &mut Frame) {
        let head = RichText::new("Configuration").monospace();

        Window::new(head).show(ctx, |ui| {
            ui.label("Enter your API_KEY for newsapi.org");

            let text_input = ui.text_edit_singleline(&mut self.config.api_key);

            if text_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Err(e) = confy::store(APP_NAME, None, HeadlinesConfig {
                    dark_mode: self.config.dark_mode,
                    api_key: self.config.api_key.to_owned(),
                }) {
                    tracing::error!("Failed saving app config {}", e);
                    frame.close()
                } else {
                    self.is_api_key_initialized = true;
                    tracing::info!("api key set");
                }
            }

            /* tracing::error!("api key: {}", self.config.api_key); */

            ui.label(RichText::new("If you have not registered for the API_KEY, head over to").small());

            let url = RichText::new("https://newsapi.org").small();

            ui.add(Hyperlink::from_label_and_url(url, "https://newsapi.org"));
        });
    }
}
