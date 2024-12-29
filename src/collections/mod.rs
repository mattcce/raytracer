pub mod angle;
pub mod colour;
pub mod matrix;
pub mod point;
pub mod vector;

// crate-level re-exports
pub(crate) use angle::*;
pub(crate) use colour::*;
pub(crate) use matrix::*;
pub(crate) use point::*;
pub(crate) use vector::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::angle::Angle;
    pub use super::colour::Colour;
    pub use super::matrix::{Matrix, Tuple4};
    pub use super::point::Point;
    pub use super::vector::Vector;
}
