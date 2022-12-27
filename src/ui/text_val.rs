use crate::prelude::*;

/// A mutable TextBuffer that will validate it's contents when changed.
///
/// The default validator will simply attempt to parse the text as `T`,
/// but a custom validator function can be provided.
pub struct ValText<T> {
    text: String,
    val: Option<T>,
    previous_val: Option<T>,
    validator: Box<dyn Fn(&str) -> Option<T>>,
}

unsafe impl<T> Send for ValText<T> {}
unsafe impl<T> Sync for ValText<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for ValText<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValText").field("text", &self.text).field("val", &self.val).finish()
    }
}

impl<T> ValText<T> {
    pub fn is_empty(&self) -> bool { self.text.is_empty() }
}

impl<T: Copy> ValText<T> {
    pub fn with_validator(validator: impl Fn(&str) -> Option<T> + 'static) -> Self {
        Self {
            text: Default::default(),
            val: Default::default(),
            previous_val: Default::default(),
            validator: Box::new(validator),
        }
    }

    pub fn get_text(&self) -> &str { self.text.as_str() }

    pub fn get_val(&self) -> Option<T> { self.val }

    pub fn is_valid(&self) -> bool { self.val.is_some() || self.text.is_empty() }

    pub fn revert(&mut self, previous_text: String) {
        self.text = previous_text;
        self.val = self.previous_val;
    }
}

impl<T: Copy + Display> ValText<T> {
    pub fn set_val(&mut self, val: T) {
        self.text = val.to_string();
        self.previous_val = self.val;
        self.val = Some(val);
    }
}

impl<T: FromStr> Default for ValText<T> {
    fn default() -> Self {
        Self {
            text: Default::default(),
            val: Default::default(),
            previous_val: Default::default(),
            validator: Box::new(|text| text.parse().ok()),
        }
    }
}

impl<T: Copy> TextBuffer for ValText<T> {
    fn is_mutable(&self) -> bool { true }

    fn as_str(&self) -> &str { self.text.as_str() }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.previous_val = self.val;
        let n = self.text.insert_text(text, char_index);
        self.val = (self.validator)(&self.text);
        n
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.previous_val = self.val;
        self.text.delete_char_range(char_range);
        self.val = (self.validator)(&self.text);
    }
}
