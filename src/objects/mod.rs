pub mod intersections;
pub mod light;
pub mod material;
pub mod ray;
pub mod transform;
pub mod shapes;

// crate-level re-exports
pub(crate) use intersections::*;
pub(crate) use light::*;
pub(crate) use material::*;
pub(crate) use shapes::*;
pub(crate) use ray::*;
pub(crate) use transform::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::intersections::{ComputedIntersect, Intersections, RawIntersect};
    pub use super::light::Light;
    pub use super::material::Material;
    pub use crate::objects::shapes::plane::Plane;
    pub use super::ray::{Intersectable, Ray};
    pub use crate::objects::shapes::sphere::Sphere;
    pub use super::transform::{Axis, Transform, TransformKind};
    pub use super::shapes::prelude::*;
}
