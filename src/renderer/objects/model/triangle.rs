#![allow(dead_code)]

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Material;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit, Vector, Vector3};
use serde::{Deserialize, Serialize, Serializer};
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            points: [
                points[0].to_homogeneous(),
                points[1].to_homogeneous(),
                points[2].to_homogeneous(),
            ],
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
            let side =
                Vector3::from_homogeneous(self.points[i] - self.points[(i + 1) % 3]).unwrap();
            let to_point = Vector3::from_homogeneous(point - self.points[i]).unwrap();
            area -= side.cross(&to_point).norm().abs() / 2.;
        }
        area.abs() <= Self::EPSILON
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        (self.points[0] - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleModel {
    mesh_file: String,

    #[serde(skip)]
    triangles: Option<Vec<Triangle>>,

    material: Material,
}

impl TriangleModel {
    pub fn new(mesh_file: String, material: Material) -> Self {
        TriangleModel {
            mesh_file,
            triangles: None,
            material,
        }
    }

    pub fn load_file(mut self) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(&self.mesh_file)?;
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
                            stl.vertices[idx].0[2].into(),
                        )
                    }),
                ))
            })
            .collect();
        self.triangles = Some(triangles);
        Ok(self)
    }
}

impl Model for TriangleModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let mut min_t = f64::INFINITY;
        let mut min_hit: Option<Hit> = None;

        self.triangles
            .as_ref()
            .unwrap()
            .iter()
            .for_each(|triangle| {
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
