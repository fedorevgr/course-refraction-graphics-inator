use crate::renderer::objects::camera::{Camera, Dimensions};
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::Vector;

pub struct Scene<'a> {
    pub active_camera: usize,
    pub objects: Vec<&'a mut Model>,
    pub cameras: Vec<&'a mut Camera>,
}

impl<'a> Scene<'a> {
    pub fn new(cam: &'a mut Camera) -> Self {
        Scene {
            cameras: vec![cam], 
            objects: Vec::new(), 
            active_camera: 0,
        }
    }
    
    pub fn add_camera(&mut self, cam: &'a mut Camera) {
        self.cameras.push(cam);
    }
    pub fn add_object(&mut self, obj: &'a mut Model) {
        self.objects.push(obj);
    }
    pub fn remove_object(&mut self, idx: usize) {
        self.objects.remove(idx);
    }
    pub fn remove_camera(&mut self, idx: usize) {
        if self.active_camera == idx {
            self.active_camera = 0;
        } else if self.active_camera > idx { 
            self.active_camera -= 1;
        }
        
        self.cameras.remove(idx);
    }
}