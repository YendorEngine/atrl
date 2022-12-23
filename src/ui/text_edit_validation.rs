use crate::prelude::*;

#[derive(Default)]
pub struct TextEditValidation {
    pub horizontal: bool,
    pub title: Option<String>,
    pub validation_fn: Option<fn(&mut String) -> bool>,
}

impl TextEditValidation {
    pub fn new() -> Self { Self::default() }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    pub fn with_validation_fn(mut self, validation_fn: fn(&mut String) -> bool) -> Self {
        self.validation_fn = Some(validation_fn);
        self
    }

    pub fn validation_ui(&self, ui: &mut egui::Ui, text: &mut String) -> egui::Response {
        if self.horizontal {
            ui.horizontal(|ui| {
                if let Some(title) = &self.title {
                    ui.label(title);
                }
                self.validation_text_edit(ui, text)
            })
            .response
        } else {
            if let Some(title) = &self.title {
                ui.label(title);
            }
            self.validation_text_edit(ui, text)
        }
    }

    fn validation_text_edit(&self, ui: &mut egui::Ui, text: &mut String) -> egui::Response {
        let previous_text = text.to_owned();
        let response = ui.text_edit_singleline(&mut *text);

        if response.changed() {
            if let Some(validate) = self.validation_fn {
                if !validate(text) {
                    *text = previous_text;
                }
            }
        }

        response
    }
}
