use news_api::NewsAPI;
use tracing;
use confy::{ConfyError, self};
use std::thread::{JoinHandle, self};
use std::sync::mpsc::{self, Receiver};
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
        Button,
        TopBottomPanel,
        menu::bar,
        RichText,
        Label,
        Window
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
    pub rx: Option<Receiver<NewsCardData>>
}

impl Headlines {
    pub fn new() -> Self {
        let config: HeadlinesConfig = confy::load(APP_NAME, None).unwrap_or_default();

        Headlines {
            is_api_key_initialized: !config.api_key.is_empty(),
            articles: vec![],
            config,
            rx: None
        }
    }

    pub fn setup(&mut self, ctx: &Context) {
        // what should be called once

        if !self.config.api_key.is_empty() {
            self.fetch_news();
        }

        self.configure_fonts(ctx);
    }

    pub fn fetch_news(&mut self) -> JoinHandle<()> {
        let api_key = self.config.api_key.clone();

        let (tx, rx) = mpsc::channel();

        let (app_tx, app_rx) = mpsc::sync_channel(1);

        let handler = thread::spawn(move || {
            if !api_key.is_empty() {
                if let Ok(response) = NewsAPI::new(&api_key).fetch() {

                    for ar in response.articles() {
                        if let Err(e) = tx.send(NewsCardData {
                            title: ar.title().to_owned(),
                            url: ar.url().to_owned(),
                            description: ar.description().map(|s| s.to_string()).unwrap_or("...".to_string())
                        }) {
                            tracing::error!("Error sending news data {}", e);
                        }
                    }
                } else {
                    tracing::error!("failed fetching news");
                }
            } else {
                loop {
                    match app_rx.recv() {
                        Ok(Msg::ApiKeySet(api_key)) => {
                            fetch_news(&api_key, &mut news_tx);
                        }
                        Err(e) => {
                            tracing::error!("failed receiving msg: {}", e);
                        }
                    }
                }
            }
        });

        self.rx = Some(rx);

        self.app_tx = Some(app_tx);

        handler

        /* self.articles.push(NewsCardData {
            title: ar.title().to_owned(),
            url: ar.url().to_owned(),
            description: ar.description().map(|s| s.to_string()).unwrap_or("...".to_string())
        }); */
    }

    pub fn preload_articles(&mut self) {
        if let Some(rx) = &self.rx {
            match rx.try_recv() {
                Ok(news_card) => {
                    self.articles.push(news_card);
                },
                Err(_) => { tracing::error!("Error receiving msg") }
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
                        let new_mode = !self.config.dark_mode;
                        if let Err(e) = self.save_config(HeadlinesConfig {
                            dark_mode: new_mode,
                            api_key: self.config.api_key.to_owned(),
                        }) {
                            tracing::error!("Failed saving app config {}", e);
                        } else {
                            self.config.dark_mode = new_mode;
                            tracing::info!("theme mode changed");
                        }
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
                if let Err(e) = self.save_config(HeadlinesConfig {
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

    fn save_config(&self, cfg: HeadlinesConfig) -> Result<(), ConfyError>{
        confy::store(APP_NAME, None, cfg)
    }
}
