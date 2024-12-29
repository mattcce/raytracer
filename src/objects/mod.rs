use crate::transform::{Transform, TransformKind};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    transform: Transform,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Transform::new(TransformKind::Identity),
        }
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sphere() {
        let sphere = Sphere::new();
        let resulting_sphere = Sphere {
            transform: Transform::new(TransformKind::Identity),
        };
        assert_eq!(sphere, resulting_sphere);
    }

    #[test]
    fn transform_sphere() {
        let mut sphere = Sphere::new();
        let transform = Transform::new(TransformKind::Translate(5.0, 0.0, 0.0));
        sphere.set_transform(&transform);
    }
}
