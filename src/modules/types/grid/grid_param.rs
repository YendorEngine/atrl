use crate::prelude::*;

pub trait GridParam: Sync + Send + 'static + FromReflect {}
impl<T: FromReflect> GridParam for T {}
