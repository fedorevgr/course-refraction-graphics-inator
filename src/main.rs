mod image_generator;
mod renderer;
mod tests;

use std::collections::HashMap as Map;
use crate::renderer::objects::camera::perspective::PerspectiveCamera;
use crate::renderer::objects::camera::{Camera, Dimensions};
use crate::renderer::objects::material::MaterialBuilder;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
use eframe::Frame;
use egui::{Context, Key};
use image::{ImageBuffer, RgbImage};

use crate::image_generator::ImageGenerator;
use crate::image_generator::implementations::one_thread::OneThreaded;
use crate::image_generator::implementations::rayon::Library;
use crate::renderer::Renderer;
use crate::renderer::implementations::global_illumination::{
    GlobalIllumination, PointLight, Solid, WithSky,
};
use crate::renderer::implementations::simple_illumination::SimpleIllumination;
use crate::renderer::objects::model::sphere::SphereModel;
use crate::renderer::objects::model::{Model, Transform};

fn main() -> Result<(), eframe::Error> {
    let camera = PerspectiveCamera::new(
        Vector::new(-0., -10., 7., 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {
            width: 1200,
            height: 800,
        },
        std::f64::consts::FRAC_PI_6,
    );

    let scene = Scene::new(vec![
        // SphereModel::new(
        //     Vector::from([0.; 4]),
        //     1.,
        //     MaterialBuilder::default()
        //         .color([0., 1., 0.].into())
        //         .roughness([1.; 3].into())
        //         .metallic([0.; 3].into())
        //         .build().unwrap()
        // ),
        SphereModel::new(
            Vector::from([-2., 0., 0., 0.]),
            1.,
            MaterialBuilder::default()
                .color([0., 0., 1.].into())
                .roughness([1.; 3].into())
                .metallic([0.; 3].into())
                .build()
                .unwrap(),
        ),
        SphereModel::new(
            Vector::from([0., 0., -30.19, 0.]),
            59. / 2.,
            MaterialBuilder::default()
                .color([0.5; 3].into())
                .roughness([1.; 3].into())
                .metallic([0.; 3].into())
                .build()
                .unwrap(),
        ),
    ]);

    // let renderer = GlobalIllumination::new(
    //     scene,
    //     vec![PointLight::new(
    //         [0., 0., 4., 0.].into(),
    //         2.,
    //         [1., 1., 1.].into(),
    //     )],
    //     3,
    //     Solid::new([0.; 3].into()),
    // );
    let renderer = SimpleIllumination::new(scene);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200., 800.0])
            .with_resizable(false)
            .with_transparent(false),
        ..Default::default()
    };

    let image_generator = Library::new(1024);

    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| Ok(Box::new(Viewer::new(cc, camera, renderer, image_generator)))),
    )
}

struct Viewer<C, G, R>
where
    R: Renderer,
    G: ImageGenerator<C, R>,
    C: Camera + Transform,
{
    camera: C,
    renderer: R,
    generator: G,
    image: egui::TextureHandle,
    actions: Map<Key, fn(&mut C)>
}

impl<C, G, R> Viewer<C, G, R>
where
    R: Renderer,
    G: ImageGenerator<C, R>,
    C: Camera + Transform,
{
    pub fn new(cc: &eframe::CreationContext<'_>, camera: C, renderer: R, generator: G) -> Self {
        let image = generator.create(&camera, &renderer); // Self::create_sample_image(); //
        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();
        let color_image = egui::ColorImage::from_rgb(size, pixels.as_slice());

        Self {
            image: cc
                .egui_ctx
                .load_texture("image", color_image, egui::TextureOptions::default()),
            camera,
            renderer,
            generator,
            actions: {
                let mut map:  Map<Key, fn(&mut C)> = Map::new();
                map.insert(Key::S, |c: &mut C| { c.reposition_by(&[0., 0., 0.1, 0.].into()); });
                map.insert(Key::W, |c: &mut C| { c.reposition_by(&[0., 0., -0.1, 0.].into()); });
                map.insert(Key::A, |c: &mut C| { c.reposition_by(&[-0.1, 0., 0., 0.].into()); });
                map.insert(Key::D, |c: &mut C| { c.reposition_by(&[0.1, 0., 0., 0.].into()); });
                map.insert(Key::V, |c: &mut C| { c.reposition_by(&[-0.0, 0.1, 0.0, 0.].into()); });
                map.insert(Key::Space, |c: &mut C| { c.reposition_by(&[-0., -0.1, 0.0, 0.].into()); });

                map.insert(Key::ArrowUp, |c: &mut C| { c.rotate_by(0.1, 0., 0.); });
                map.insert(Key::ArrowDown, |c: &mut C| { c.rotate_by(-0.1, 0., 0.); });
                map.insert(Key::ArrowLeft, |c: &mut C| { c.rotate_by(0., -0.05, 0.); });
                map.insert(Key::ArrowRight, |c: &mut C| { c.rotate_by(0., 0.05, 0.); });

                map
            }
        }
    }

    fn render_new(&self, ctx: &Context) -> egui::TextureHandle {
        let image = self.generator.create(&self.camera, &self.renderer);
        let for_texture = egui::ColorImage::from_rgb(
            [image.width() as usize, image.height() as usize],
            image.as_raw(),
        );
        ctx.load_texture("image", for_texture, egui::TextureOptions::default())
    }
}
impl<C, G, R> eframe::App for Viewer<C, G, R>
where
    R: Renderer,
    G: ImageGenerator<C, R>,
    C: Camera + Transform,
{
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if let Some(action) = ctx.input(|i| {
            for (key, action) in &self.actions {
                if i.key_down(key.clone()) {
                    return Some(action);
                }
            }
            None
        }) {
            action(&mut self.camera);
            self.image = self.render_new(&ctx);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let available_size = ui.available_size();
                ui.image((self.image.id(), available_size));
            });
    }
}
