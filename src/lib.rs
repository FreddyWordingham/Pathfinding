#![allow(dead_code)]

mod characters;
mod input;
mod map;

pub mod prelude {
    pub use crate::characters::*;
    pub use crate::input::*;
    pub use crate::map::*;
}
