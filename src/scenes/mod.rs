pub mod canvas;

// crate-level re-exports
pub use canvas::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::canvas;
    pub use super::canvas::Canvas;
}
