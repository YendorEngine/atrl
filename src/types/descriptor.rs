use crate::prelude::*;

/////////////////////
// Descriptor
/////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Descriptor<T: Copy> {
    data: T,
    description: &'static str,
}

impl<T: Copy> Descriptor<T> {
    pub fn new(description: &'static str, data: T) -> Self { Self { data, description } }

    pub fn get_data(&self) -> T { self.data }

    pub fn get_description(&self) -> &'static str { self.description }
}

//////////////////////////////////////////
// Impls
//////////////////////////////////////////

impl<T: Copy> From<Descriptor<T>> for String {
    fn from(size_descriptor: Descriptor<T>) -> Self { size_descriptor.description.to_string() }
}

impl<T: Copy> From<&Descriptor<T>> for String {
    fn from(size_descriptor: &Descriptor<T>) -> Self { size_descriptor.description.to_string() }
}

impl<T: Display + Copy> Display for Descriptor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.description, self.data)
    }
}

//////////////////////////////////////////
// Descriptors
//////////////////////////////////////////

pub type SizeDescriptor = Descriptor<UVec2>;
pub type WindowDescriptor = Descriptor<WindowMode>;

impl SizeDescriptor {
    pub fn to_display(&self) -> String { format!("{}: {}x{}", self.description, self.data.x, self.data.y) }
}
