use crate::prelude::*;

#[macro_export]
macro_rules! impl_new{
  ($to:ty,$($v:ident: $t:ty),*)  => {
      impl $to {
          pub fn new($($v: $t),*) -> $to
          {
              Self {
                  $($v),*
              }
          }
      }
  };
}

#[macro_export]
macro_rules! impl_default {
    ($to:ty) => {
        impl Default for $to {
            fn default() -> Self { Self::new() }
        }
    };
}

#[macro_export]
macro_rules! insert_resource {
    ($r:expr) => {
        |mut commands: Commands| {
            commands.insert_resource($r);
        }
    };

    ($commands:ident, $r:expr) => {
        $commands.insert_resource($r);
    };
}

#[macro_export]
macro_rules! remove_resource {
    ($t:ty) => {
        |mut commands: Commands| {
            commands.remove_resource::<$t>();
        }
    };

    ($commands:ident, $t:ty) => {
        $commands.remove_resource::<$t>();
    };
}

#[macro_export]
macro_rules! spawn_component {
    ($c:expr) => {
        |mut commands: Commands| {
            commands.spawn($c);
        }
    };

    ($commands:ident, $c:expr) => {
        $commands.spawn($c);
    };
}
