use std::fmt::{self, Debug, Display};

use crate::prelude::*;

#[derive(Clone)]
pub struct Prht {
    seed: u64,
    xxhash: Xxh3,
}

impl Prht {
    pub const fn new(seed: u64) -> Self {
        let xxhash = Xxh3Builder::new().with_seed(seed).build();
        Self { seed, xxhash }
    }

    pub fn get<X: Into<i64>, Y: Into<i64>, Z: Into<i64>>(&mut self, x: X, y: Y, z: Z) -> u64 {
        let x = x.into();
        let y = y.into();
        let z = z.into();
        self.xxhash.reset();
        self.xxhash.update(&x.to_be_bytes());
        self.xxhash.update(&y.to_be_bytes());
        self.xxhash.update(&z.to_be_bytes());
        self.xxhash.digest()
    }
}

impl Debug for Prht {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Prht {{ seed:{} }}", self.seed))
    }
}

impl Display for Prht {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Prht {{ seed:{} }}", self.seed))
    }
}
