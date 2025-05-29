use nalgebra::Rotation3;

use super::ray::{Vector, Matrix};

#[derive(Debug)]
pub struct Dimensions {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug)]
pub struct Camera {
    pos: Vector,
    dir: Vector,
    
    fov: f64,
    pub dimensions: Dimensions
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
        let fov_x  = self.fov;
        let fov_y = self.fov / self.dimensions.width as f64 * self.dimensions.height as f64;
        
        let d_alpha = fov_x / (self.dimensions.width-1) as f64;
        let d_beta = fov_y / (self.dimensions.height-1) as f64;
        
        let lim_x = -fov_x / 2.;
        let lim_y = -fov_y / 2.;
      
        let mut row: i64 = 0;
        let mut col: i64 = -1;
        
        move || {
            if col + 1 < self.dimensions.width {
                col += 1;
            }
            else {
                row += 1;
                col = 0;
            }
            // println!("{:#?}, {:#?}; {:#?}, {:#?}", row, col, lim_y + d_beta * (row as f64), lim_x + d_alpha * (col as f64));
            let res_yaw: Vector = 
            Rotation3::from_euler_angles(
                0.,
                0.,
                lim_x + d_alpha * (col as f64)
            ) * self.dir;
            let res: Vector = Rotation3::from_euler_angles(
                0.,
                lim_y + d_beta * (row as f64),
                0.
            ) * res_yaw;
            println!("{:#?}, {:#?}, {:#?}", row, col, res);
            res
        }
    }
}