pub mod intersections;
pub mod light;
pub mod material;
pub mod plane;
pub mod ray;
pub mod sphere;
pub mod transform;

// crate-level re-exports
pub(crate) use intersections::*;
pub(crate) use light::*;
pub(crate) use material::*;
pub(crate) use plane::*;
pub(crate) use ray::*;
pub(crate) use sphere::*;
pub(crate) use transform::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::intersections::{ComputedIntersect, Intersections, RawIntersect};
    pub use super::light::Light;
    pub use super::material::Material;
    pub use super::plane::Plane;
    pub use super::ray::{Intersectable, Ray};
    pub use super::sphere::Sphere;
    pub use super::transform::{Axis, Transform, TransformKind};
}
