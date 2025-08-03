#![allow(dead_code)]

use std::error::Error;
use std::fmt::Debug;

use std::fs::OpenOptions;
use std::path::Path;

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Material;
use crate::renderer::objects::ray::{Ray, Unit, Vector, Vector3};

pub trait Model {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub normal: Unit,
    pub points: [Vector; 3],
    area: f64,
}

impl Triangle {
    const EPSILON: f64 = 1e-9;
    pub fn new(normal: Unit, points: &[Vector3; 3]) -> Self {
        Triangle {
            normal,
            points: [points[0].to_homogeneous(), points[1].to_homogeneous(), points[2].to_homogeneous()],
            area: (points[2] - points[0])
                .cross(&(points[1] - points[0]))
                .norm()
                .abs()
                / 2.,
        }
    }

    pub fn point_in(&self, point: &Vector) -> bool {
        let mut area = self.area;
        for i in 0..3 {
            let side = Vector3::from_homogeneous(self.points[i] - self.points[(i + 1) % 3]).unwrap();
            let to_point = Vector3::from_homogeneous(point - self.points[i]).unwrap_or_else( || {
                println!("Stop");
                Vector3::from_homogeneous(point.clone()).unwrap()
            });
            area -= side.cross(&to_point).norm().abs() / 2.;
        }
        area.abs() <= Self::EPSILON
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        (self.points[0] - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal)
    }
}

#[derive(Debug, Clone)]
pub struct TriangleModel {
    triangles: Vec<Triangle>,
    material: Material,
}

impl TriangleModel {
    pub fn new(triangles: Vec<Triangle>, material: Material) -> Self {
        TriangleModel {
            triangles,
            material,
        }
    }

    pub fn from_stl<Str: AsRef<Path>>(
        filename: Str,
        material: Material,
    ) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(filename)?;
        let stl = stl_io::read_stl(&mut file)?;
        let triangles = stl
            .faces
            .iter()
            .filter_map(|face| {
                let norm = Vector::new(
                    face.normal[0].into(),
                    face.normal[1].into(),
                    face.normal[2].into(),
                    0.,
                );

                if (norm.magnitude_squared() - 1.).abs() >= 0.0001 {
                    return None;
                }

                Some(Triangle::new(
                    Unit::new_unchecked(norm),
                    &face.vertices.map(|idx| {
                        Vector3::new(
                            stl.vertices[idx].0[0].into(),
                            stl.vertices[idx].0[1].into(),
                            stl.vertices[idx].0[2].into()
                        )
                    }),
                ))
            })
            .collect();

        Ok(Self::new(triangles, material))
    }
}

impl Model for TriangleModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let mut min_t = f64::INFINITY;
        let mut min_hit: Option<Hit> = None;

        self.triangles.iter().for_each(|triangle| {
            if -triangle.normal.dot(&ray.direction) < 0. {
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
    material: Material,
}

impl SphereModel {
    pub fn new(center: Vector, radius: f64, material: Material) -> SphereModel {
        SphereModel {
            center,
            radius_sq: radius * radius,
            material,
        }
    }
}

impl Model for SphereModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let b = 2. * ray.direction.dot(&(ray.origin - self.center));
        let c = (self.center - ray.origin).magnitude_squared() - self.radius_sq;

        let d = b * b - 4. * c;
        if d < 0. {
            None
        } else {
            let t = (-b - d.sqrt()) / 2.;
            let hit_pos = ray.origin + ray.direction.scale(t);
            Some(Hit::new(
                t,
                hit_pos,
                &self.material,
                Unit::new_normalize(hit_pos - self.center),
            ))
        }
    }
}

pub struct TorusModel {
    pub r: f64,
    pub k: f64,
    pub material: Material,
}

impl Model for TorusModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let step = 0.05;
        let dir = ray.direction.normalize();
        for i in 0..60 {
            let t = step * i as f64;
            let p = ray.origin + dir * t;

            if (p.magnitude_squared() + self.r - self.k).powi(2)
                < 4. * self.r * (p.x.powi(2) + p.y.powi(2))
            {
                return Some(Hit::new(
                    t,
                    p,
                    &self.material,
                    Unit::new_normalize(p - (p - Vector::new(0., 0., p.z, 0.))),
                ));
            }
        }
        None
    }
}
