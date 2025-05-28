use nalgebra::Rotation3;
use super::ray::{Vector, Matrix};

pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

pub struct Camera {
    pos: Vector,
    dir: Vector,
    
    fov: f64,
    dimensions: Dimensions
}

impl Camera {
    pub fn new(pos: Vector, dir: Vector, fov: f64, dims: Dimensions) -> Self {
        Camera {
            pos,
            dir,
            fov,
            dimensions: dims,
        }
    }
    pub fn pixel_vectors(&self) -> impl FnMut() -> Vector {
        let angle_x = self.fov;
        let angle_y = self.fov / self.dimensions.width as f64 * self.dimensions.height as f64;
        
        let mut row_beg_vector: Vector = Rotation3::from_euler_angles(
            0.0,
            angle_y / 2., angle_x / 2.
        ) * self.dir;
        let mut col_vector: Vector = row_beg_vector.clone();

        let mut row: usize = 0;
        let mut col: usize = 0;
        
        let delta_x = Rotation3::from_euler_angles(0., 0., angle_x / self.dimensions.width as f64);
        let delta_y = Rotation3::from_euler_angles(0., angle_y / self.dimensions.height as f64, 0.);
        move || {
            if col == self.dimensions.width {
                row += 1;
                col = 0;
                
                row_beg_vector = delta_y * row_beg_vector;
                col_vector = row_beg_vector.clone();
            }
            else { 
                col += 1;
                col_vector = delta_x * col_vector;
            }
            col_vector
        }
    }
}