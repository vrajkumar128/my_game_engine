use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SpriteData {
    pub b: i32,
    pub g: i32,
    pub height: i32,
    pub r: i32,
    pub width: i32,
    pub x: i32,
    pub y: i32,
}