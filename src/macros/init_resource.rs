#[macro_export]
macro_rules! init_resource {
    ($commands:ident, $r:ident) => {
        $commands.init_resource::<$r>();
    };

    ($r:ident) => {
        |mut commands: Commands| {
            commands.init_resource::<$r>();
        }
    };
}
