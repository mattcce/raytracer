use std::ops::Index;

use crate::ray::Sphere;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersect<'a> {
    t: f64,
    object: &'a Sphere,
}

impl<'a> Intersect<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Intersect<'a> {
        Intersect { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Sphere {
        self.object
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Intersections<'a>(Vec<Intersect<'a>>);

impl<'a> Intersections<'a> {
    pub fn add(&mut self, intersect: Intersect<'a>) -> () {
        for (i, v) in self.0.iter_mut().enumerate() {
            if intersect.t < v.t {
                self.0.insert(i, intersect);
                break;
            }
        }
    }

    pub fn hit(&self) -> &Intersect {
        for v in &self.0 {
            if v.t >= 0.0 {
                return v;
            }
        }
        panic!();
    }
}

impl<'a> From<Vec<Intersect<'a>>> for Intersections<'a> {
    fn from(mut vec: Vec<Intersect<'a>>) -> Intersections<'a> {
        assert_ne!(vec.len(), 0);
        vec.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections(vec)
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersect<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Sphere;

    #[test]
    fn create_intersect() {
        let sphere = Sphere::new();
        let intersect = Intersect::new(1.0, &sphere);
        let resulting_intersect = Intersect {
            t: 1.0,
            object: &sphere,
        };
        assert_eq!(intersect.t, resulting_intersect.t);
        assert!(std::ptr::eq(intersect.object, resulting_intersect.object));
    }

    #[test]
    fn create_intersections() {
        let sphere = Sphere::new();
        let intersect1 = Intersect::new(1.0, &sphere);
        let intersect2 = Intersect::new(2.0, &sphere);
        let intersect3 = Intersect::new(3.0, &sphere);
        let resulting_intersections = Intersections(vec![
            Intersect::new(1.0, &sphere),
            Intersect::new(2.0, &sphere),
            Intersect::new(3.0, &sphere),
        ]);
        assert_eq!(
            Intersections::from(vec![intersect2, intersect3, intersect1]),
            resulting_intersections
        );
    }

    #[test]
    fn add_intersections() {
        let sphere = Sphere::new();
        let intersect1 = Intersect::new(1.0, &sphere);
        let intersect2 = Intersect::new(2.0, &sphere);
        let intersect3 = Intersect::new(3.0, &sphere);
        let mut intersections = Intersections::from(vec![intersect1, intersect3]);
        intersections.add(intersect2);
        let resulting_intersections = Intersections(vec![
            Intersect::new(1.0, &sphere),
            Intersect::new(2.0, &sphere),
            Intersect::new(3.0, &sphere),
        ]);
        assert_eq!(intersections, resulting_intersections);
    }

    #[test]
    fn intersections_hit() {
        let sphere = Sphere::new();
        let intersect1 = Intersect::new(-1.0, &sphere);
        let intersect2 = Intersect::new(2.0, &sphere);
        let intersect3 = Intersect::new(3.0, &sphere);
        let intersections = Intersections::from(vec![intersect1, intersect2, intersect3]);
        let resulting_hit = &intersections.0[1];
        assert!(std::ptr::eq(intersections.hit(), resulting_hit));
    }
}
