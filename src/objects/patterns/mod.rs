pub mod gradient;
pub mod pattern;
pub mod ring;
pub mod solid;
pub mod stripe;
pub mod checker;

// crate-level re-exports
pub use gradient::*;
pub use pattern::*;
pub use ring::*;
pub use solid::*;
pub use stripe::*;
pub use checker::*;

// public re-exports (through crate::prelude)
pub mod prelude {
    pub use super::gradient::Gradient;
    pub use super::pattern::Pattern;
    pub use super::ring::Ring;
    pub use super::solid::Solid;
    pub use super::stripe::Stripe;
    pub use super::checker::Checker;
}
