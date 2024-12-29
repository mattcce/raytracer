pub mod plane;
pub mod shape;
pub mod sphere;

// crate-level re-exports
pub(crate) use plane::*;
pub(crate) use shape::*;
pub(crate) use sphere::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::plane::Plane;
    pub use super::shape::Shape;
    pub use super::sphere::Sphere;
}
