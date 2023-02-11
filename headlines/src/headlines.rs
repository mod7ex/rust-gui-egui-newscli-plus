use std::{iter::FromIterator};
use eframe::{
    egui::{
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
        Button, TopBottomPanel, menu::bar, RichText, Label
    }
};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

pub struct Headlines {
    articles: Vec<NewsCardData>
}

impl Headlines {
    pub fn new() -> Headlines {
        let articles = Vec::from_iter((0..20).map(|i| NewsCardData {
            title: format!("title {i}"),
            description: format!("description {i}"),
            url: format!("https://example.com/{i}"),
        }));

        Headlines { articles }
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
            /* (Small, FontId::new(10.0, Proportional)), */
        ]
        .into();
        ctx.set_style(style);
    }

    pub fn render_news(&self, ui: &mut Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            
            // title
            let title = format!("▶ {}", a.title);
            ui.colored_label(WHITE, title);
            
            ui.add_space(PADDING);

            // description
            if ui.add(Button::new(&a.description)).clicked() {
                println!("clicked --------- ")
            };

            // render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more ...", &a.url))
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub fn render_top_panel(&self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
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
                        println!("closed --------- ");
                    };
                    if ui.add(Button::new(RichText::new("🔄").heading())).clicked() {
                        println!("refresh ");
                    };
                    if ui.add(Button::new(RichText::new("🌙").heading())).clicked() {
                        println!("switch theme ");
                    };
                });
            })
        });
    }
}
