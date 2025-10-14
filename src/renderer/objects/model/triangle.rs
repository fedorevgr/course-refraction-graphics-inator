#![allow(dead_code)]

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Material;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit, Unit3, Vector, Vector3};
use serde::{Deserialize, Serialize, Serializer};
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::Arc as Rc;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub normal: Unit,
    pub indexes: [usize; 3],

    points: Rc<Vec<Vector>>,
    area: f64,
}

impl Triangle {
    const EPSILON: f64 = 1e-6;
    pub fn new(normal: Unit, indexes: [usize; 3], points: Rc<Vec<Vector>>) -> Self {
        Triangle {
            normal,
            indexes,
            area: (Vector3::from_homogeneous(points[indexes[2]] - points[indexes[0]]).unwrap())
                .cross(&Vector3::from_homogeneous(points[indexes[1]] - points[indexes[0]]).unwrap())
                .norm()
                .abs()
                / 2.,
            points,
        }
    }

    fn get_point(&self, idx: usize) -> &Vector {
        &self.points[self.indexes[idx]]
    }

    pub fn point_in(&self, point: &Vector) -> bool {
        let mut area = self.area;
        for i in 0..3 {
            let side = self.get_point(i) - self.get_point((i + 1) % 3);
                // self.points[self.indexes[i]] - self.points[self.indexes[(i + 1) % 3]];
            let to_point = point- self.get_point(i); // self.indexes[i];
            area -= Vector3::from_homogeneous(side).unwrap().cross(&Vector3::from_homogeneous(to_point).unwrap()).norm().abs() / 2.;
        }
        area.abs() <= Self::EPSILON
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        (self.get_point(0) - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal)
    }
}

#[derive(Debug, Clone,Default)]
pub struct BoundingVolume{
    center: Vector,
    radius_sq: f64,
}



impl BoundingVolume {
    pub fn new(center: Vector, radius: f64) -> Self {
        BoundingVolume { center, radius_sq: radius * radius }
    }

    fn hit(&self, ray: &Ray) -> bool {
        let b = 2. * ray.direction.dot(&(ray.origin - self.center));
        let c = (self.center - ray.origin).magnitude_squared() - self.radius_sq;

        let d = b * b - 4. * c;
        if d < 0. {
            false
        } else {
            let t = -b + d.sqrt();
            // + sqrt(d) -- as point can be inside
            if t < 0. {
                return false;
            }

            true
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleModel {
    mesh_file: String,

    #[serde(skip)]
    triangles: Vec<Triangle>,
    #[serde(skip)]
    points: Rc<Vec<Vector>>,

    #[serde(skip)]
    center: Vector,

    material: Material,
    #[serde(skip)]
    bounding_box: BoundingVolume,
}

impl TriangleModel {
    pub fn new(mesh_file: String, material: Material) -> Self {
        TriangleModel {
            mesh_file,
            triangles: Vec::new(),
            points: Vec::new().into(),
            material,
            center: Vector::zeros(),
            bounding_box: BoundingVolume::default(),
        }
    }

    pub fn load_file(mut self) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(&self.mesh_file)?;
        let stl = stl_io::read_stl(&mut file)?;

        self.points = stl.vertices.iter().map(|vertex| Vector::new(vertex.0[0] as f64, vertex.0[1] as f64, vertex.0[2] as f64, 0.0)).collect::<Vec<_>>().into();
        self.triangles = stl.faces.iter().filter_map(|face| {
                let norm = Unit::new_unchecked(Vector::new(
                    face.normal[0].into(),
                    face.normal[1].into(),
                    face.normal[2].into(),
                    0.0
                ));

                if (norm.magnitude_squared() - 1.).abs() >= 0.0001 {
                    return None;
                }

                Some(Triangle::new(
                    norm,
                    face.vertices.clone(),
                    self.points.clone()
                    ),
                )
            })
            .collect::<Vec<_>>().into();

        let mut bounds = [[0., 0.], [0., 0.], [0., 0.]]; // x, y, z [min, max]
        self.points.iter().for_each(|point| {
            bounds[0][0] = point.x.min(bounds[0][0]);
            bounds[0][1] = point.x.max(bounds[0][1]);

            bounds[1][0] = point.y.min(bounds[1][0]);
            bounds[1][1] = point.y.max(bounds[1][1]);

            bounds[2][0] = point.z.min(bounds[2][0]);
            bounds[2][1] = point.z.max(bounds[2][1]);
        });

        self.center = Vector::new((bounds[0][1] + bounds[0][0]) / 2., (bounds[1][1] + bounds[1][0]) / 2.0, (bounds[2][1] + bounds[2][0]) / 2., 0.);
        self.bounding_box = BoundingVolume::new(self.center, (Vector::new(bounds[0][1],bounds[1][1], bounds[2][1], 0.) - self.center).magnitude());

        Ok(self)
    }
}

impl Model for TriangleModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        if ! self.bounding_box.hit(ray) {
            return None; // todo темный потому что логика пересечения не та
        }

        let mut min_t = f64::INFINITY;
        let mut min_hit: Option<Hit> = None;

        (*self.triangles)
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
