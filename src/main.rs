#![allow(unused_imports)]

mod image_generator;
mod renderer;
mod tests;

use crate::renderer::objects::camera::perspective::PerspectiveCamera;
use crate::renderer::objects::camera::{Camera, Dimensions};
use crate::renderer::objects::material::MaterialBuilder;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
use eframe::Frame;
use egui::accesskit::Role::Search;
use egui::{Context, Key};
use image::{ImageBuffer, RgbImage};
use std::collections::HashMap as Map;
use rand::SeedableRng;
use crate::image_generator::ImageGenerator;
use crate::image_generator::implementations::one_thread::OneThreaded;
use crate::image_generator::implementations::rayon::Library;
use crate::renderer::Renderer;
use crate::renderer::implementations::global_illumination::{
    GlobalIllumination, PointLight, Solid, WithSky,
};
use crate::renderer::implementations::sampling::{Black, Sampling};
use crate::renderer::implementations::simple_illumination::SimpleIllumination;
use crate::renderer::objects::model::sphere::SphereModel;
use crate::renderer::objects::model::{Model, Move, Rotate};

fn main() -> Result<(), eframe::Error> {
    let camera = PerspectiveCamera::new(
        Vector::new(5., -5., 5., 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {
            width: 1200,
            height: 800,
        },
        std::f64::consts::FRAC_PI_6 / 1.,
    );

    let scene = Scene::new(vec![
        SphereModel::new(
            Vector::from([0.; 4]),
            1.,
            MaterialBuilder::default()
                .color([0., 1., 0.].into())
                .roughness([1.; 3].into())
                .metallic([0.; 3].into())
                .build()
                .unwrap(),
        ),
        SphereModel::new(
            Vector::from([-2., 0., 0., 0.]),
            1.,
            MaterialBuilder::default()
                .color([0.2, 0.2, 0.2].into())
                .roughness([0.2; 3].into())
                .metallic([0.8; 3].into())
                .ambient([0.; 3].into())
                .build()
                .unwrap(),
        ),
        SphereModel::new(
            Vector::from([-1., 1., 0., 0.]),
            0.5,
            MaterialBuilder::default()
                .color([0.5, 0.2, 0.2].into())
                .roughness([0.8; 3].into())
                .metallic([0.2; 3].into())
                .ambient([0.; 3].into())
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

    let renderer = GlobalIllumination::new(
        scene,
        vec![PointLight::new(
            [0., 0., 4., 0.].into(),
            20.,
            [1., 1., 1.].into(),
        )],
        5,
        Solid::new([0.05; 3].into()),
        //WithSky{}
    );
    // let renderer = SimpleIllumination::new(scene);
    // let renderer = Sampling::new(scene, Black{}, 2, rand_xoshiro::Xoroshiro128PlusPlus::seed_from_u64(0), 5);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200., 800.0])
            .with_resizable(false)
            .with_transparent(false),
        ..Default::default()
    };

    let image_generator = Library::new(1024);
    // let image_generator = OneThreaded {};
    // let _ = image_generator.create(&camera, &renderer).save("artifacts/Test.png").unwrap();
    // Ok(())

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
    C: Camera + Move + Rotate,
{
    camera: C,
    renderer: R,
    generator: G,
    image: egui::TextureHandle,

    actions: Map<Key, fn(&mut C, f64)>,

    frame_rate: f64,
}

impl<C, G, R> Viewer<C, G, R>
where
    R: Renderer,
    G: ImageGenerator<C, R>,
    C: Camera + Move + Rotate,
{
    const POSITION_STEP: f64 = 3.;
    const ROTATION_STEP: f64 = 0.3;
    const FORWARD: Vector = Vector::new(0., 0., -Self::POSITION_STEP, 0.);
    const BACKWARD: Vector = Vector::new(0., 0., Self::POSITION_STEP, 0.);
    const RIGHT: Vector = Vector::new(Self::POSITION_STEP, 0., 0., 0.);
    const LEFT: Vector = Vector::new(-Self::POSITION_STEP, 0., 0., 0.);
    const UP: Vector = Vector::new(0., Self::POSITION_STEP, 0., 0.);
    const DOWN: Vector = Vector::new(0., -Self::POSITION_STEP, 0., 0.);

    pub fn new(cc: &eframe::CreationContext<'_>, camera: C, renderer: R, generator: G) -> Self {
        let time = std::time::Instant::now();
        let image = generator.create(&camera, &renderer);
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
                let mut map: Map<Key, fn(&mut C, f64)> = Map::new();
                map.insert(Key::S, |c: &mut C, k: f64| {
                    c.reposition_by(&(Self::BACKWARD * k));
                });
                map.insert(Key::W, |c: &mut C, k: f64| {
                    c.reposition_by(&(Self::FORWARD * k));
                });
                map.insert(Key::A, |c: &mut C, k: f64| {
                    c.reposition_by(&(Self::LEFT * k));
                });
                map.insert(Key::D, |c: &mut C, k: f64| {
                    c.reposition_by(&(Self::RIGHT * k));
                });
                map.insert(Key::V, |c: &mut C, k: f64| {
                    c.reposition_by(&(Self::UP * k));
                });
                map.insert(Key::Space, |c: &mut C, k: f64| {
                    c.reposition_by(&(Self::DOWN * k));
                });

                map.insert(Key::ArrowUp, |c: &mut C, k: f64| {
                    c.rotate_by(Self::ROTATION_STEP * k, 0., 0.);
                });
                map.insert(Key::ArrowDown, |c: &mut C, k: f64| {
                    c.rotate_by(-Self::ROTATION_STEP * k, 0., 0.);
                });
                map.insert(Key::ArrowLeft, |c: &mut C, k: f64| {
                    c.rotate_by(0., Self::ROTATION_STEP * k, 0.);
                });
                map.insert(Key::ArrowRight, |c: &mut C, k: f64| {
                    c.rotate_by(0., -Self::ROTATION_STEP * k, 0.);
                });

                map
            },
            frame_rate: time.elapsed().as_secs_f64(),
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
    C: Camera + Move + Rotate,
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {

        let mut actions = Vec::with_capacity(self.actions.len());
        ctx.input(|i| {
            i.keys_down.clone().iter().for_each(|k| {
                if let Some(action) = self.actions.get(k) {
                    actions.push(action);
                }
            });
        });
        if !actions.is_empty() {
            actions.iter().for_each(|action| {
                action(&mut self.camera, self.frame_rate);
            });

            let time = std::time::Instant::now();
            self.image = self.render_new(ctx);
            self.frame_rate = time.elapsed().as_secs_f64();
            dbg!(1. / self.frame_rate);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let available_size = ui.available_size();
                ui.image((self.image.id(), available_size));
            });

    }
}
