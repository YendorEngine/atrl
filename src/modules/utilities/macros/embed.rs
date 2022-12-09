#[macro_export]
macro_rules! embedded_resource {
    ($resource_name : ident, $filename : expr) => {
        pub const $resource_name: &'static [u8] = include_bytes!($filename);
    };
}
