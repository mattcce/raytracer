pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod shape;
pub mod sphere;

// crate-level re-exports
pub(crate) use cube::*;
pub(crate) use cylinder::*;
pub(crate) use plane::*;
pub(crate) use shape::*;
pub(crate) use sphere::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::cube::Cube;
    pub use super::cylinder::Cylinder;
    pub use super::plane::Plane;
    pub use super::shape::Shape;
    pub use super::sphere::Sphere;
}
