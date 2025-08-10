use crate::renderer::objects::camera::{Camera, Dimensions};
use crate::renderer::objects::ray::{Matrix, Ray, Unit, Vector};

#[derive(Debug)]
pub struct FishEyeCamera {
    pos: Vector,
    dir_angles: Vector,

    fov: f64,
    pub dimensions: Dimensions,
}

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
            Unit::new_normalize(matrix_pitch * (matrix_yaw * Vector::new(0., 0., -1., 0.))),
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
