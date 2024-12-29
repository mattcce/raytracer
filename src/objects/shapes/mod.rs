pub mod plane;
pub mod sphere;
pub mod shape;

// crate-level re-exports
pub(crate) use plane::*;
pub(crate) use sphere::*;
pub(crate) use shape::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::shape::Shape;
    pub use super::plane::Plane;
    pub use super::sphere::Sphere;
}
