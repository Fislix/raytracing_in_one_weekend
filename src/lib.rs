pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod point3;
pub mod random;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use color::Color;
pub use hittable::{HitRecord, Hittable};
pub use interval::Interval;
pub use material::{Lambertian, Material, Scatter};
pub use point3::Point3;
pub use ray::Ray;
pub use vec3::Vec3;

pub const MAX_VALUE: u32 = 255;
