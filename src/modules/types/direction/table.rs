use std::{
    mem,
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    slice,
};

use crate::prelude::*;
pub type DirectionTableIter<'a, T> = slice::Iter<'a, T>;
pub type DirectionTableIterMut<'a, T> = slice::IterMut<'a, T>;
macro_rules! make_direction_table {
    (
      $table_type:ident,
      $enumerate_type:ident,
      $enumerate_mut_type:ident,
      $direction_type:ident,
      $direction_into_iter:ident,
      $direction_iter:ident,
      $count:expr
  ) => {
        #[derive(
            Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        pub struct $table_type<T> {
            values: [T; $count],
        }
        pub type $enumerate_type<'a, T> = std::iter::Zip<$direction_iter, DirectionTableIter<'a, T>>;
        pub type $enumerate_mut_type<'a, T> = std::iter::Zip<$direction_iter, DirectionTableIterMut<'a, T>>;
        impl<T> $table_type<T> {
            pub fn new_fn<F: FnMut($direction_type) -> T>(mut f: F) -> Self {
                let values = unsafe {
                    let mut values: [T; $count] = MaybeUninit::uninit().assume_init();
                    for i in 0..$count {
                        values[i] = f(mem::transmute(i as u8));
                    }
                    values
                };
                Self { values }
            }

            pub const fn new_array(values: [T; $count]) -> Self { Self { values } }

            pub fn set(&mut self, direction: $direction_type, value: T) {
                self.values[direction as usize] = value;
            }

            pub const fn get(&self, direction: $direction_type) -> &T { &self.values[direction as usize] }

            pub fn get_mut(&mut self, direction: $direction_type) -> &mut T {
                &mut self.values[direction as usize]
            }

            pub fn iter(&self) -> DirectionTableIter<T> { self.values.iter() }

            pub fn iter_mut(&mut self) -> DirectionTableIterMut<T> { self.values.iter_mut() }

            pub const fn directions(&self) -> $direction_iter { $direction_iter::new() }

            pub fn enumerate(&self) -> $enumerate_type<T> { self.directions().zip(self.iter()) }

            pub fn enumerate_mut(&mut self) -> $enumerate_mut_type<T> {
                self.directions().zip(self.iter_mut())
            }
        }
        impl<T: Clone> $table_type<T> {
            pub fn new_clone(value: T) -> Self {
                let values = unsafe {
                    let mut values: [T; $count] = MaybeUninit::uninit().assume_init();
                    for i in 0..$count {
                        values[i] = value.clone();
                    }
                    values
                };
                Self { values }
            }
        }
        impl<T: Default> $table_type<T> {
            pub fn new_default() -> Self {
                let values = unsafe {
                    let mut values: [T; $count] = MaybeUninit::uninit().assume_init();
                    for i in 0..$count {
                        values[i] = Default::default();
                    }
                    values
                };
                Self { values }
            }
        }
        impl<T> Index<$direction_type> for $table_type<T> {
            type Output = T;

            fn index(&self, index: $direction_type) -> &Self::Output { self.values.index(index as usize) }
        }
        impl<T> IndexMut<$direction_type> for $table_type<T> {
            fn index_mut(&mut self, index: $direction_type) -> &mut Self::Output {
                self.values.index_mut(index as usize)
            }
        }
    };
}
make_direction_table!(
    DirectionTable,
    DirectionTableEnumerate,
    DirectionTableEnumerateMut,
    Direction,
    Directions,
    DirectionIter,
    NUM_DIRECTIONS
);
make_direction_table!(
    CardinalDirectionTable,
    CardinalDirectionTableEnumerate,
    CardinalDirectionTableEnumerateMut,
    CardinalDirection,
    CardinalDirections,
    CardinalDirectionIter,
    NUM_CARDINAL_DIRECTIONS
);
make_direction_table!(
    OrdinalDirectionTable,
    OrdinalDirectionTableEnumerate,
    OrdinalDirectionTableEnumerateMut,
    OrdinalDirection,
    OrdinalDirections,
    OrdinalDirectionIter,
    NUM_ORDINAL_DIRECTIONS
);
