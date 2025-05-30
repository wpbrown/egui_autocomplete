use std::collections::BTreeSet;

use eframe::egui;
use egui::{TextEdit, Ui, Vec2};
use egui_autocomplete::AutoCompleteTextEdit;

static_toml::static_toml!(
    static PARENT_TOML = include_toml!("../Cargo.toml");
);

const STARTER_LIST: &str = r#"writer
seat
dog
worker
grade
face
ahead
immediately
dance
too
equipment
alike
noun
soil
floor
both
copper
tune
say
plural
bark
exciting
hold
price
coat
creature
news
across
strange
mud
force
her
told
hour
natural
finally
dot
mysterious
rapidly
larger
race
treated
"#;

pub struct TemplateApp {
    auto_complete1: AutoCompleteExample,
    auto_complete2: AutoCompleteExample,
    max_suggestions: usize,
    highlight: bool,
    multiple_words: bool,
    popup_on_focus: bool,
}

struct AutoCompleteExample {
    multi_input: String,
    search_field: String,
}

impl AutoCompleteExample {
    fn update(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut Ui,
        max_suggestions: usize,
        highlight_matches: bool,
        multiple_words: bool,
        popup_on_focus: bool,
    ) {
        let inputs = self.multi_input.lines().collect::<BTreeSet<_>>();
        ui.add(
            AutoCompleteTextEdit::new(&mut self.search_field, inputs)
                .max_suggestions(max_suggestions)
                .highlight_matches(highlight_matches)
                .multiple_words(multiple_words)
                .popup_on_focus(popup_on_focus),
        );
        ui.add(TextEdit::multiline(&mut self.multi_input));
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            auto_complete1: AutoCompleteExample {
                multi_input: STARTER_LIST.to_string(),
                search_field: Default::default(),
            },
            auto_complete2: AutoCompleteExample {
                multi_input: Default::default(),
                search_field: Default::default(),
            },
            max_suggestions: 10,
            highlight: false,
            multiple_words: false,
            popup_on_focus: false,
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading(format!("egui_autocomplete v{} demo", PARENT_TOML.package.version));
                ui.label(
                    r#"Enter text in the single line entry for auto_complete.
Add new lines in the multiline textbox to add to the autocomplete menu.
Use arrow keys to select completion.
Use enter, tab or mouseclick to apply completion."#,
                );
            });
            ui.separator();
            ui.vertical_centered_justified(|ui| ui.heading("Options"));
            ui.vertical_centered_justified(|ui| {
                ui.checkbox(&mut self.highlight, "Highlight matches")
                    .on_hover_text(
                    "If highlight is set, matching characters will be highlighted in the drop down",
                );
                ui.checkbox(&mut self.popup_on_focus, "Popup on focus")
                    .on_hover_text(
                    "If set, will show autocomplete popup on focus instead of requiring one character to be typed",
                );
                ui.checkbox(&mut self.multiple_words, "Multiple words")
                    .on_hover_text(
                        "If set, autocomplete each word instead of the entire field at once",
                    );
                ui.add(egui::DragValue::new(&mut self.max_suggestions).prefix("Max suggestions: "))
                    .on_hover_text(
                        "This determines the maximum number options to show in the drop down",
                    );
                if self.max_suggestions == 0 {
                    ui.label("Setting that to 0 basically disables the autocomplete");
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    ui.available_size() / Vec2::new(2., 1.),
                    egui::Layout::top_down(egui::Align::Max),
                    |ui| {
                        self.auto_complete1.update(
                            ctx,
                            ui,
                            self.max_suggestions,
                            self.highlight,
                            self.multiple_words,
                            self.popup_on_focus,
                        );
                    },
                );
                ui.separator();
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        self.auto_complete2.update(
                            ctx,
                            ui,
                            self.max_suggestions,
                            self.highlight,
                            self.multiple_words,
                            self.popup_on_focus,
                        );
                    },
                );
            });
        });
    }
}
