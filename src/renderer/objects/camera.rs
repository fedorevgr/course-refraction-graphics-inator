#![allow(dead_code)]

use crate::renderer::objects::ray::{Matrix, Ray, Vector, Unit, Vector3};

pub trait Camera {
    fn gen_ray(&self, u: usize, v: usize) -> Ray;

    fn get_dimensions(&self) -> &Dimensions;
}

#[derive(Debug, Clone)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct FishEyeCamera {
    pos: Vector,
    dir_angles: Vector,

    fov: f64,
    pub dimensions: Dimensions,
}

// todo: forgot what it does
impl FishEyeCamera {
    pub fn new(pos: Vector, yaw: f64, pitch: f64, fov: f64, dims: Dimensions) -> Self {
        FishEyeCamera {
            pos,
            dir_angles: Vector::new(0., pitch, yaw, 0.),
            fov,
            dimensions: dims,
        }
    }

    pub fn get_vector(&self, col: usize, row: usize) -> Option<Ray> {
        if (col > self.dimensions.width) || (row > self.dimensions.height) {
            return None;
        }

        let fov_y = self.fov / self.dimensions.width as f64 * self.dimensions.height as f64;

        let lim_x = -self.fov / 2.;
        let lim_y = -fov_y / 2.;

        let delta_col = self.fov / (self.dimensions.width - 1) as f64;
        let delta_row = fov_y / (self.dimensions.height - 1) as f64;

        let matrix_yaw =
            Matrix::from_euler_angles(0., 0., self.dir_angles.z + lim_x + delta_col * col as f64);
        let matrix_pitch =
            Matrix::from_euler_angles(0., self.dir_angles.y + lim_y + delta_row * row as f64, 0.);

        Some(Ray::new(
            self.pos,
            Unit::new_normalize(
                matrix_pitch * (matrix_yaw * Vector::new(0., 0., -1., 0.)),
            )
        ))
    }
}

impl Camera for FishEyeCamera {
    fn gen_ray(&self, u: usize, v: usize) -> Ray {
        self.get_vector(u, v).unwrap()
    }
    fn get_dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
}

pub struct PerspectiveCamera {
    pos: Vector,
    pitch: f64,
    yaw: f64,
    fov: f64,
    pub dims: Dimensions,
}

impl PerspectiveCamera {
    pub fn new(pos: Vector, target: Vector, dims: Dimensions, fov: f64) -> Self {
        let dir = Unit::new_normalize(target - pos);
        let pitch = dir.dot(&Vector::z_axis()).acos();
        let mut yaw = dir.dot(&Vector::y_axis()).acos();

        if dir.dot(&Vector::x_axis()) > 0. {
            yaw *= -1.;
        }

        PerspectiveCamera {
            pitch,
            yaw,
            pos,
            dims,
            fov
        }
    }

    fn project(&self, x: usize, y: usize) -> Vector {
        let aspect = self.dims.height as f64 / self.dims.width as f64;
        let tan_hor_fov = (self.fov / 2.).tan();
        let tan_ver_fov = tan_hor_fov * aspect;
        let hor_step = tan_hor_fov / (self.dims.width as f64 / 2.);
        let ver_step = -tan_ver_fov / (self.dims.height as f64 / 2.);

        Vector::new(hor_step * x as f64 - tan_hor_fov, ver_step * y as f64 + tan_ver_fov, -1., 0.)
    }

    fn transition(&self) -> Matrix {
        Matrix::new_rotation(Vector3::new(self.pitch, 0., self.yaw))
    }

}

impl Camera for PerspectiveCamera {
    fn gen_ray(&self, u: usize, v: usize) -> Ray {
        let uv = self.project(u, v);
        let projected = self.transition() * uv;

        Ray::new(
            self.pos,
            Unit::new_normalize(projected)
        )
    }

    fn get_dimensions(&self) -> &Dimensions {
        &self.dims
    }
}
