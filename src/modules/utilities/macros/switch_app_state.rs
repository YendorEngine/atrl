#[macro_export]
macro_rules! switch_app_state {
    ($e:expr) => {
        move |mut commands: Commands| {
            commands.insert_resource(iyes_loopless::prelude::NextState($e));
        }
    };

    ($commands:ident, $s:expr) => {
        $commands.insert_resource(iyes_loopless::prelude::NextState($s));
    };
}
