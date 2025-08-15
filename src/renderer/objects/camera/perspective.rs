use crate::renderer::objects::camera::{Camera, Dimensions};
use crate::renderer::objects::ray::{Matrix, Ray, Unit, Vector, Vector3};

#[derive(Clone)]
pub struct PerspectiveCamera {
    pos: Vector,
    pitch: f64,
    yaw: f64,
    fov: f64,

    pitch_matrix: Matrix,
    yaw_matrix: Matrix,

    pub dims: Dimensions,
}

impl PerspectiveCamera {
    const DEFAULT_DIR: Unit = Unit::new_unchecked(Vector::new(0., 0., -1., 0.));

    pub fn new(pos: Vector, target: Vector, dims: Dimensions, fov: f64) -> Self {
        let dir = Unit::new_normalize(target - pos);

        let [pitch, yaw] = Self::define_angles(&dir);

        PerspectiveCamera {
            pitch,
            yaw,
            pos,
            dims,
            fov,
            pitch_matrix: Matrix::new_rotation(Vector3::new(pitch, 0., 0.)),
            yaw_matrix: Matrix::new_rotation(Vector3::new(0., 0., yaw))
        }
    }

    fn project(&self, x: usize, y: usize) -> Vector {
        let aspect = self.dims.height as f64 / self.dims.width as f64;
        let tan_hor_fov = (self.fov / 2.).tan();
        let tan_ver_fov = tan_hor_fov * aspect;
        let hor_step = tan_hor_fov / (self.dims.width as f64 / 2.);
        let ver_step = -tan_ver_fov / (self.dims.height as f64 / 2.);

        Vector::new(
            hor_step * x as f64 - tan_hor_fov,
            ver_step * y as f64 + tan_ver_fov,
            -1.,
            0.,
        )
    }

    fn transition(&self, v: &Vector) -> Vector {
        self.yaw_matrix * (self.pitch_matrix * v)
    }

    fn define_angles(dir: &Unit) -> [f64; 2] {
        let mut yaw = (dir.dot(&Vector::y_axis()) / (1. - dir.z * dir.z).sqrt()).acos();

        if dir.dot(&Vector::x_axis()) > 0. {
            yaw *= -1.;
        }

        let pitch = Self::DEFAULT_DIR.dot(dir).acos();

        [pitch, yaw]
    }
}

impl Camera for PerspectiveCamera {
    fn gen_ray(&self, u: usize, v: usize) -> Ray {
        let uv = self.project(u, v);
        let projected = self.transition(&uv);

        Ray::new(self.pos, Unit::new_normalize(projected), 1.)
    }

    fn get_dimensions(&self) -> &Dimensions {
        &self.dims
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::renderer::objects::camera::{Dimensions};
    use super::PerspectiveCamera;
    use crate::renderer::objects::ray::{Matrix, Unit, Vector, Vector3};

    #[test]
    fn test_straight_angles_definition() {
        let target_dir = Unit::new_normalize(Vector::new(0.000, -1.000, 0.000, 0.000));

        let [pitch, yaw] = PerspectiveCamera::define_angles(&target_dir);

        assert_relative_eq!(pitch, 90f64.to_radians(), max_relative = 0.001);
        assert_relative_eq!(yaw, 180f64.to_radians(), max_relative = 0.001);
    }

    #[test]
    fn test_angles_definition() {
        let target_dir = Unit::new_normalize(Vector::new(1.000, 1.000, 0.000, 0.000));

        let [pitch, yaw] = PerspectiveCamera::define_angles(&target_dir);

        assert_relative_eq!(pitch, 90f64.to_radians(), max_relative = 0.001);
        assert_relative_eq!(yaw, -45f64.to_radians(), max_relative = 0.001);
    }

    #[test]
    fn test_pitch_definition() {
        let target_dir = Unit::new_normalize(Vector::new(0.000, 1.000, 1.000, 0.000));
        let [pitch, _] = PerspectiveCamera::define_angles(&target_dir);

        assert_relative_eq!(pitch, 135f64.to_radians(), max_relative = 0.001);
    }

    #[test]
    fn test_rotation_chain() {
        let target_dir = Unit::new_normalize(Vector::new(0.000, -1.000, 0.000, 0.000));
        let [pitch, yaw] = PerspectiveCamera::define_angles(&target_dir);

        let pitch_rot = Matrix::new_rotation(Vector3::new(pitch, 0., 0.))
            * PerspectiveCamera::DEFAULT_DIR.into_inner();
        let res_dir = Matrix::new_rotation(Vector3::new(0., 0., yaw)) * pitch_rot;

        assert_relative_eq!(res_dir, target_dir.into_inner(), epsilon = 0.001);
    }

    #[test]
    fn test_overall() {
        let pos = Vector::new(5., -5.0, 5., 0.);
        let target = Vector::new(0., 0., 0., 0.);
        let cam = PerspectiveCamera::new(
            pos,
            target,
            Dimensions {width: 40, height: 30},
            std::f64::consts::FRAC_PI_6
        );

        let target_dir = Unit::new_normalize(target - pos);
        let res_dir = cam.transition(&PerspectiveCamera::DEFAULT_DIR);

        assert_relative_eq!(cam.yaw, 45f64.to_radians(), epsilon = 0.001);
        assert_relative_eq!(res_dir, target_dir.into_inner(), epsilon = 0.001);
    }
}
