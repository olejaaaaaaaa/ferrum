use std::collections::HashMap;
use image::DynamicImage;

pub struct ImageManager {
    pub image: HashMap<&'static str, DynamicImage>
}

impl ImageManager {
    pub fn new() -> Self {
        Self { image: HashMap::new() }
    }
}