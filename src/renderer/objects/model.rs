#![allow(dead_code)]

use std::fmt::Debug;

use crate::renderer::objects::ray::{Ray, Vector, Unit};
use crate::renderer::objects::material::Material;
use crate::renderer::objects::hit::Hit;

pub trait Model
{
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub normal: Unit,
    pub points: [Vector; 3],
    area: f64
}

impl Triangle {
    pub fn new(normal: Unit, points: &[Vector; 3], idx: [usize; 3]) -> Self {
        Triangle {
            normal,
            points: [points[idx[0]], points[idx[1]], points[idx[2]]],
            area: (points[idx[2]] - points[idx[0]]).cross(&(points[idx[1]] - points[idx[0]])).norm().abs() / 2.
        }
    }

    pub fn point_in(&self, point: &Vector) -> bool {
        let mut area = self.area;
        for i in 0..3 {
            let from = self.points[i];
            let next = self.points[(i + 1) % 3];
            area -= (from - next).cross(&(point - from)).norm().abs() / 2.;
        }
        area.abs() < f64::EPSILON
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        (self.points[0] - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal)
    }
}

#[derive(Debug, Clone)]
pub struct TriangleModel {
    triangles: Vec<Triangle>,
    material: Material
}

impl TriangleModel {

}

impl Model for TriangleModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {

        let mut min_t = f64::INFINITY;
        let mut min_hit: Option<Hit> = None;

        self.triangles.iter().for_each(|triangle| {
            if triangle.normal.dot(&ray.direction) < 0. {
                return;
            }

            let t = triangle.intersect(ray);

            if t < 0. || min_t <= t {
                return;
            }

            let hit_pos = ray.origin + ray.direction.scale(t);

            if triangle.point_in(&hit_pos) {
                min_t = t;
                min_hit = Some(Hit::new(t, hit_pos, &self.material, triangle.normal));
            }
        });

        min_hit
    }
}


#[derive(Debug, Clone)]
pub struct SphereModel {
    center: Vector,
    radius_sq: f64,
    material: Material
}

impl SphereModel {
    pub fn new(center: Vector, radius: f64, material: Material) -> SphereModel {
        SphereModel {center, radius_sq: radius * radius, material}
    }
}

impl Model for SphereModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let b = 2. * ray.direction.dot(&(ray.origin - self.center));
        let c  = (self.center - ray.origin).magnitude_squared() - self.radius_sq;

        let d = b * b - 4. * c;
        if d < 0. {
            None
        }
        else {
            let t = (-b - d.sqrt()) / 2.;
            let hit_pos = ray.origin + ray.direction.scale(t);
            Some(
                Hit::new(
                    t,
                    hit_pos,
                    &self.material,
                Unit::new_normalize(hit_pos - self.center)
                )
            )
        }
    }
}

pub struct TorusModel {
    pub r: f64,
    pub k: f64,
    pub material: Material
}

impl Model for TorusModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let step = 0.05;
        let dir = ray.direction.normalize();
        for i in 0..60 {
            let t = step * i as f64;
            let p = ray.origin + dir * t;

            if (p.magnitude_squared() + self.r - self.k).powi(2) < 4. * self.r * (p.x.powi(2) + p.y.powi(2))
            {
                return Some(
                        Hit::new(
                        t,
                        p,
                        &self.material,
                        Unit::new_normalize(p - (p - Vector::new(0., 0., p.z, 0.)))
                    )
                );
            }
        }
        None
    }
}

