pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod shape;
pub mod smooth_triangle;
pub mod sphere;
pub mod triangle;

// crate-level re-exports
pub(crate) use cone::*;
pub(crate) use cube::*;
pub(crate) use cylinder::*;
pub(crate) use plane::*;
pub(crate) use shape::*;
pub(crate) use smooth_triangle::*;
pub(crate) use sphere::*;
pub(crate) use triangle::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::cone::Cone;
    pub use super::cube::Cube;
    pub use super::cylinder::Cylinder;
    pub use super::plane::Plane;
    pub use super::shape::Shape;
    pub use super::smooth_triangle::SmoothTriangle;
    pub use super::sphere::Sphere;
    pub use super::triangle::Triangle;
}
