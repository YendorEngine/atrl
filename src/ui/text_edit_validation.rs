use super::ValText;
use crate::prelude::*;

pub trait WidgetLayout {
    fn layout(self, ui: &mut Ui) -> Response;
}

#[derive(Default)]
pub struct TextEditValidation<In> {
    input: In,
    pub enabled: bool,
    pub horizontal: bool,
    pub label: Option<WidgetText>,
    pub previous_text: Option<String>,
    pub validator: Option<Box<dyn Fn(&str) -> bool>>,
}

impl<In> TextEditValidation<In> {
    pub fn new(input: In) -> Self {
        Self {
            input,
            label: None,
            enabled: true,
            validator: None,
            horizontal: false,
            previous_text: None,
        }
    }

    pub fn new_with_label(label: impl Into<WidgetText>, input: In) -> Self {
        Self {
            input,
            enabled: true,
            validator: None,
            horizontal: false,
            previous_text: None,
            label: Some(label.into()),
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    // pub fn validation(&self, res: Response) -> Response {
    //     if self.input.is_valid() {
    //         self.previous_text = Some(self.input.get_text().to_owned());
    //     } else {
    //         self.input.revert(self.previous_text.unwrap());
    //     }
    // }
}

//////////////// ValText Implementation ////////////////

impl<'a, T: Copy> WidgetLayout for TextEditValidation<&'a mut ValText<T>> {
    fn layout(mut self, ui: &mut Ui) -> Response {
        self.label.map(|l| ui.label(l));

        let previous_text = match self.previous_text {
            Some(prev) => {
                if prev != self.input.get_text() {
                    self.previous_text = Some(prev.to_owned());
                }
                prev
            },
            None => {
                let text = self.input.get_text().to_owned();
                self.previous_text = Some(text.to_owned());
                text
            },
        };

        let res = ui.add_enabled(self.enabled, TextEdit::singleline(self.input));
        if res.changed() && !self.input.is_valid() {
            self.input.revert(previous_text);
        }

        res
    }
}

impl<'a, T: Copy> Widget for TextEditValidation<&'a mut ValText<T>> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.horizontal {
            ui.horizontal(|ui| self.layout(ui)).inner
        } else {
            self.layout(ui)
        }
    }
}

//////////////// String Implementation ////////////////

impl<'a> WidgetLayout for TextEditValidation<&'a mut String> {
    fn layout(self, ui: &mut Ui) -> Response {
        self.label.map(|l| ui.label(l));

        let previous_text = self.input.to_owned();
        let res = ui.add_enabled(self.enabled, TextEdit::singleline(self.input));

        let validator = self.validator.unwrap();
        if res.changed() && !validator(self.input) {
            *self.input = previous_text;
        }

        res
    }
}

impl<'a> TextEditValidation<&'a mut String> {
    #[must_use = "Text Edit Validation needs a validation function"]
    pub fn with_validator(mut self, validator: impl Fn(&str) -> bool + 'static) -> Self {
        self.validator = Some(Box::new(validator));
        self
    }
}

impl<'a> Widget for TextEditValidation<&'a mut String> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.horizontal {
            ui.horizontal(|ui| self.layout(ui)).inner
        } else {
            self.layout(ui)
        }
    }
}
