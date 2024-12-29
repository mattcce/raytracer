use crate::objects::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TaggedPixel {
    pub index: [usize; 2],
    pub blend_weight: f64,
}

impl TaggedPixel {
    pub fn new(index: [usize; 2], blend_weight: f64) -> TaggedPixel {
        TaggedPixel {
            index,
            blend_weight,
        }
    }

    pub fn index(&self) -> [usize; 2] {
        self.index
    }

    pub fn blend_weight(&self) -> f64 {
        self.blend_weight
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaggedRay {
    pub ray: Ray,
    pub pixels: Vec<TaggedPixel>,
}

impl TaggedRay {
    pub fn new(ray: Ray, pixels: Vec<TaggedPixel>) -> TaggedRay {
        TaggedRay { ray, pixels }
    }

    pub fn ray(&self) -> Ray {
        self.ray
    }

    pub fn pixels(&self) -> &Vec<TaggedPixel> {
        &self.pixels
    }
}

pub trait RayGenerator: IntoIterator<Item = TaggedRay> {
    fn canvas_size(&self) -> (usize, usize);
}
