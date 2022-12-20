pub struct Resolution {
    pub size: (f32, f32),
    pub name: String,
}

impl Resolution {
    pub fn new(name: &str, width: f32, height: f32) -> Self {
        Self {
            size: (width, height),
            name: name.to_string(),
        }
    }
}
