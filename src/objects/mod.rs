pub mod intersections;
pub mod light;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod transform;

// crate-level re-exports
pub(crate) use intersections::*;
pub(crate) use light::*;
pub(crate) use material::*;
pub(crate) use ray::*;
pub(crate) use sphere::*;
pub(crate) use transform::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::intersections::{Intersect, Intersections};
    pub use super::light::PointLight;
    pub use super::material::Material;
    pub use super::ray::{Intersectable, Ray};
    pub use super::sphere::Sphere;
    pub use super::transform::{Axis, Transform, TransformKind};
}
