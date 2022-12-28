use crate::{prelude::*, test::*};

#[test]
fn universe_generation_main() {
    Test {
        frames: 1,
        // setup: todo!(),
        check: |_app, ()| {},
        ..Default::default()
    }
    .run()
}
