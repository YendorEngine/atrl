use super::ValText;
use crate::prelude::*;

pub struct OptionEditor<'a, In, Out> {
    input: In,
    label: WidgetText,
    on_changed: Box<dyn FnMut(Out) + 'a>,

    enabled: bool,
    inital_focus: bool,
    allow_invalid: bool,
}

impl<'a, In, Out> OptionEditor<'a, In, Out> {
    #[inline(always)]
    pub fn new(text: impl Into<WidgetText>, input: In, changed: impl FnMut(Out) + 'a) -> Self {
        Self {
            input,
            enabled: true,
            label: text.into(),
            inital_focus: false,
            allow_invalid: true,
            on_changed: Box::new(changed),
        }
    }

    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[inline]
    pub fn allow_invalid(mut self, allow_invalid: bool) -> Self {
        self.allow_invalid = allow_invalid;
        self
    }

    #[inline]
    pub fn inital_focus(mut self, inital_focus: bool) -> Self {
        self.inital_focus = inital_focus;
        self
    }
}

impl<'a, T: Copy> Widget for OptionEditor<'a, &'a mut ValText<T>, T> {
    fn ui(mut self, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if !self.input.is_valid() {
                ui.style_mut().visuals.override_text_color = Some(Color32::DARK_RED);
            }

            // Paint Label
            ui.label(self.label);

            // Paint Input
            let previous_text = self.input.get_text().to_owned();
            let response = {
                let response =
                    ui.add_enabled(self.enabled, TextEdit::singleline(self.input)).kbgp_navigation();

                if self.inital_focus {
                    response.kbgp_initial_focus()
                } else {
                    response
                }
            };

            if response.changed() && self.input.is_valid() {
                if let Some(x) = self.input.get_val() {
                    (self.on_changed)(x);
                }
            } else if response.changed() && !self.allow_invalid {
                self.input.revert(previous_text);
            }
        })
        .response
    }
}

impl<'a> Widget for OptionEditor<'a, bool, bool> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            if ui.add_enabled(self.enabled, Checkbox::new(&mut self.input, self.label)).changed() {
                (self.on_changed)(self.input);
            }
        })
        .response
    }
}

impl<'a, T: Copy + Debug> Widget for OptionEditor<'a, &'a mut [ValText<T>], (usize, T)> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.collapsing(self.label, |ui| {
            for (i, input) in self.input.iter_mut().enumerate() {
                ui.add(
                    OptionEditor::new(format!("Tier {}", i + 1), input, |x| {
                        (self.on_changed)((i, x))
                    })
                    .enabled(self.enabled),
                );
            }
        })
        .header_response
    }
}
