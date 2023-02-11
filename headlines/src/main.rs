use eframe::{
    run_native, 
    App,
    Frame,
    NativeOptions,
    egui::{
        Context,
        CentralPanel,
        ScrollArea,
        Vec2, Ui, Separator, TopBottomPanel, Label, Hyperlink, RichText
    }
};

mod headlines;

use headlines::{Headlines, PADDING};

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines")
    });

    ui.add_space(PADDING);

    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            
            ui.add(Label::new("API source: newsapi.org"));
            
            ui.add(
                Hyperlink::from_label_and_url(
                    RichText::new("Made with egui").monospace(),
                    "https://github.com/emilk/egui"
                )
            );

            ui.add_space(10.);
        })
    });
}

impl App for Headlines {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {

        self.render_top_panel(ctx);

        self.configure_fonts(ctx);

        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);

            ScrollArea::both().show(ui, |ui| {
                self.render_news(ui);
            });

            render_footer(ctx);
        });
    }
}

fn main() {
    let app = Headlines::new();

    let mut native_options = NativeOptions::default();

    native_options.initial_window_size = Some(Vec2::new(540., 960.));

    run_native("News API", native_options, Box::new(|_| Box::new(app))).unwrap();
}