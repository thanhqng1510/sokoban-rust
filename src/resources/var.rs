use crate::components::{WallColor, WallShape, FloorMaterial, FloorType};
use std::collections::HashMap;


pub struct BackgroundColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor {
            r: 0.7, g: 0.7, b: 0.7, a: 1.
        }
    }
}

pub struct ComponentTemplateData {
    pub data: HashMap<String, String>
}

impl Default for ComponentTemplateData {
    fn default() -> Self {
        ComponentTemplateData {
            data: vec![
                (String::from("wall_color"), WallColor::Brown.to_string()),
                (String::from("wall_shape"), WallShape::Round.to_string()),
                (String::from("floor_material"), FloorMaterial::Dirt.to_string()),
                (String::from("floor_type"), FloorType::Gravel.to_string())
            ].into_iter().collect()
        }
    }
}
