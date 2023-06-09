use super::{
    color::{self, Color},
    HitRecord, Ray, SolidColor, Texture, TextureEnum, Vec3,
};

use enum_dispatch::enum_dispatch;
use rand::prelude::*;

use std::sync::Arc;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

#[enum_dispatch]
pub enum MaterialEnum {
    Lambertian,
    UniformScatter,
    Metal,
    Dialectric,
}

#[enum_dispatch(MaterialEnum)]
pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    albedo: Arc<TextureEnum>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        let texture = Arc::new(SolidColor::new(albedo).into());
        Lambertian { albedo: texture }
    }

    pub fn from_texture(texture: Arc<TextureEnum>) -> Lambertian {
        Lambertian { albedo: texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered_ray = Ray::new(hit_record.point, scatter_direction, Some(ray_in.time()));

        Some(Scatter {
            ray: scattered_ray,
            attenuation: self
                .albedo
                .value(hit_record.u, hit_record.v, hit_record.point),
        })
    }
}

pub struct UniformScatter {
    albedo: Color,
}

impl UniformScatter {
    pub fn new(albedo: Color) -> UniformScatter {
        UniformScatter { albedo }
    }
}

impl Material for UniformScatter {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = Vec3::random_in_hemisphere(hit_record.normal);

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered_ray = Ray::new(hit_record.point, scatter_direction, Some(ray_in.time()));

        Some(Scatter {
            ray: scattered_ray,
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Metal {
        let fuzziness = fuzziness.min(1.0);
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected_direction = Vec3::reflect(ray_in.direction(), hit_record.normal);
        let scattered_ray = Ray::new(
            hit_record.point,
            reflected_direction + self.fuzziness * Vec3::random_in_unit_sphere(),
            None,
        );
        let does_hit = Vec3::dot(scattered_ray.direction(), hit_record.normal) > 0.0;
        if !does_hit {
            return None;
        }

        Some(Scatter {
            ray: scattered_ray,
            attenuation: self.albedo,
        })
    }
}

pub struct Dialectric {
    refractive_index: f64,
}

impl Dialectric {
    pub fn new(refractive_index: f64) -> Dialectric {
        Dialectric { refractive_index }
    }

    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = match hit_record.on_front_face {
            true => 1.0 / self.refractive_index,
            false => self.refractive_index,
        };

        let unit_direction = ray_in.direction().normalized();
        let cos_theta = 1.0_f64.min(Vec3::dot(-unit_direction, hit_record.normal));
        let sin_theta = (1.0_f64 - cos_theta * cos_theta).sqrt();

        let mut rng = thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dialectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                Vec3::reflect(unit_direction, hit_record.normal)
            } else {
                Vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
            };

        let scattered_ray = Ray::new(hit_record.point, direction, Some(ray_in.time()));
        let attenuation = color::WHITE;

        Some(Scatter {
            ray: scattered_ray,
            attenuation,
        })
    }
}
