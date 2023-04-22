use super::{Color, Point3};

use enum_dispatch::enum_dispatch;

use std::sync::Arc;

#[enum_dispatch]
pub enum TextureEnum {
    SolidColor,
    CheckerBoard,
}

#[enum_dispatch(TextureEnum)]
pub trait Texture {
    fn value(&self, u: f64, v: f64, hit_point: Point3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        SolidColor::new(Color::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _hit_point: Point3) -> Color {
        self.color
    }
}

pub struct CheckerBoard {
    even: Arc<TextureEnum>,
    odd: Arc<TextureEnum>,
}

impl CheckerBoard {
    pub fn new(even: Arc<TextureEnum>, odd: Arc<TextureEnum>) -> CheckerBoard {
        CheckerBoard { even, odd }
    }

    pub fn from_colors(even: Color, odd: Color) -> CheckerBoard {
        CheckerBoard {
            even: Arc::new(SolidColor::new(even).into()),
            odd: Arc::new(SolidColor::new(odd).into()),
        }
    }
}

impl Texture for CheckerBoard {
    fn value(&self, u: f64, v: f64, hit_point: Point3) -> Color {
        let sines = [hit_point.x(), hit_point.y(), hit_point.z()]
            .map(|coord| (10.0 * coord).sin())
            .into_iter()
            .reduce(|acc, e| acc * e)
            .unwrap();
        match sines < 0.0 {
            true => self.odd.value(u, v, hit_point),
            false => self.even.value(u, v, hit_point),
        }
    }
}