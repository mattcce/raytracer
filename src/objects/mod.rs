pub mod bounds;
pub mod group;
pub mod intersections;
pub mod light;
pub mod material;
pub mod patterns;
pub mod ray;
pub mod shapes;
pub mod transform;

// crate-level re-exports
pub(crate) use bounds::*;
pub(crate) use group::*;
pub(crate) use intersections::*;
pub(crate) use light::*;
pub(crate) use material::*;
pub(crate) use patterns::*;
pub(crate) use ray::*;
pub(crate) use shapes::*;
pub(crate) use transform::*;

// public re-exports (through crate::prelude)
pub(super) mod prelude {
    pub use super::patterns::prelude::*;
    pub use super::shapes::prelude::*;

    pub use super::group::Group;
    pub use super::intersections::{Coordinates, HitRegister, Intersect};
    pub use super::light::Light;
    pub use super::material::Material;
    pub use super::ray::Ray;
    pub use super::transform::{Axis, Transform, TransformKind};
}
