use crate::collections::{Point, Vector};
use crate::objects::{
    BoundingBox, Bounds, Coordinates, Material, PrimitiveShape, Ray, Shape, Transform,
    Transformable,
};
use crate::utils::{Buildable, ConsumingBuilder, EPSILON};

#[derive(Debug)]
pub struct Triangle {
    frame_transformation: Transform,
    material: Material,
    vertices: [Point; 3],
    edges: [Vector; 2],
    normal: Vector,
    bounds: Bounds,
}

impl Triangle {
    pub fn vertices(&self) -> [Point; 3] {
        self.vertices
    }

    pub fn edges(&self) -> [Vector; 2] {
        self.edges
    }

    pub fn normal(&self) -> Vector {
        self.normal
    }
}

impl PrimitiveShape for Triangle {
    fn frame_transformation(&self) -> &Transform {
        &self.frame_transformation
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn local_normal_at(&self, _local_point: Point, _: Option<(f64, f64)>) -> Vector {
        self.normal
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<Coordinates> {
        let dir_cross_e2 = local_ray.direction.cross(self.edges[1]);
        let det = self.edges[0].dot(dir_cross_e2);
        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = local_ray.origin - self.vertices[0];
        let u = f * p1_to_origin.dot(dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(self.edges[0]);
        let v = f * local_ray.direction.dot(origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * self.edges[1].dot(origin_cross_e1);
        vec![t].iter().map(|&t| Coordinates::new(t, None)).collect()
    }

    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

#[derive(Debug, Default)]
pub struct TriangleBuilder {
    frame_transformation: Option<Transform>,
    material: Option<Material>,
    vertices: Option<[Point; 3]>,
}

impl TriangleBuilder {
    pub fn set_frame_transformation(mut self, frame_transformation: Transform) -> TriangleBuilder {
        self.frame_transformation = Some(frame_transformation);
        self
    }

    pub fn set_material(mut self, material: Material) -> TriangleBuilder {
        self.material = Some(material);
        self
    }

    pub fn set_vertices(mut self, vertices: [Point; 3]) -> TriangleBuilder {
        self.vertices = Some(vertices);
        self
    }
}

impl Buildable for Triangle {
    type Builder = TriangleBuilder;

    fn builder() -> Self::Builder {
        TriangleBuilder::default()
    }
}

impl ConsumingBuilder for TriangleBuilder {
    type Built = Triangle;

    fn build(self) -> Self::Built {
        let frame_transformation = self.frame_transformation.unwrap_or_default();
        let material = self.material.unwrap_or_default();
        let [v1, v2, v3] = self.vertices.unwrap();
        let e1 = v2 - v1;
        let e2 = v3 - v1;
        let normal = e2.cross(e1).normalise();
        let bounds = Bounds::new(
            BoundingBox::from_anchors(vec![v1, v2, v3]).transform(&frame_transformation),
        );

        let triangle = Triangle {
            frame_transformation,
            material,
            vertices: [v1, v2, v3],
            edges: [e1, e2],
            normal,
            bounds,
        };
        triangle
    }
}

impl Into<Shape> for Triangle {
    fn into(self) -> Shape {
        Shape::Primitive(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_ray_parallel_to_triangle() {
        let vertices = [
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        ];
        let triangle = Triangle::builder().set_vertices(vertices).build();
        let ray = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(triangle.local_intersect(&ray).len(), 0);
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let vertices = [
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        ];
        let triangle = Triangle::builder().set_vertices(vertices).build();
        let ray = Ray::new(Point::new(1.0, 1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(triangle.local_intersect(&ray).len(), 0);
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let vertices = [
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        ];
        let triangle = Triangle::builder().set_vertices(vertices).build();
        let ray = Ray::new(Point::new(-1.0, 1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(triangle.local_intersect(&ray).len(), 0);
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let vertices = [
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        ];
        let triangle = Triangle::builder().set_vertices(vertices).build();
        let ray = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(triangle.local_intersect(&ray).len(), 0);
    }

    #[test]
    fn ray_intersects_triangle() {
        let vertices = [
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        ];
        let triangle = Triangle::builder().set_vertices(vertices).build();
        let ray = Ray::new(Point::new(0.0, 0.5, -2.0), Vector::new(0.0, 0.0, 1.0));
        let t_values = triangle.local_intersect(&ray);
        assert_eq!(t_values.len(), 1);
        assert_eq!(t_values[0].t(), 2.0);
    }
}
