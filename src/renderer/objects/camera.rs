use crate::renderer::objects::ray::{Matrix, Ray, Vector};
use nalgebra::Unit;

#[derive(Debug, Clone)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct Camera {
    pos: Vector,
    dir_angles: Vector,

    fov: f64,
    pub dimensions: Dimensions,
}

impl Camera {
    pub fn new(pos: Vector, yaw: f64, pitch: f64, fov: f64, dims: Dimensions) -> Self {
        Camera {
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
                matrix_pitch * (matrix_yaw * Vector::new(1., 0., 0., 0.)),
            )
        ))
    }

    pub fn pixel_vectors(&self) -> impl FnMut() -> Ray {
        let mut row: usize = 0;
        let mut col: usize = 0;

        move || {
            if col >= self.dimensions.width {
                row += 1;
                col = 0;
            }

            col += 1;
            self.get_vector(col - 1, row).unwrap()
        }
    }
}
