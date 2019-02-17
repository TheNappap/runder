
mod renderer;
mod math;
mod cg_tools;
mod camera;
mod objects;
mod scene;
mod settings;
mod units;
mod thread_pool;

use std::sync::{Arc};
use std::f64::consts::{PI,FRAC_PI_2};

use settings::Settings;
use objects::*;
use camera::{PerspectiveCamera};
use scene::{SceneGraph};
use math::{Point, Vector, Normal, Direction, RotationAxis};
use cg_tools::{SamplingTechnique,Transformation};
use units::Color;


fn main() {
    let settings = settings::Settings {
        screen_width: 800,
        screen_height: 600,
        chunk_width: 80,
        chunk_height: 60,
        amt_threads: 6,
        aa_multi_sample: 1,
        light_sampling_technique: SamplingTechnique::Stratified{multi_sample: 1, seed: 0.0}
    };

    let settings = Arc::new(settings);
    let camera = default_camera(settings.clone());
    let scene = default_scene(settings.clone());

    renderer::render(settings, camera, scene);
}

fn default_camera(settings: Arc<Settings>) -> PerspectiveCamera
{
    let position = Point::new(0.0,2.0,-2.0);
    let direction = Direction::new(0.0,-1.0,5.0);
    let up = Direction::up();
    PerspectiveCamera::new(settings, position,direction,up,60.0)
}

fn default_scene(settings: Arc<Settings>) -> SceneGraph{
    let mut scene_graph = SceneGraph::new(settings);
    //scene_graph.add_object(Box::new(Sphere::new(Transformation::new().translate(Vector::new(0.0,0.0,3.0)), Box::new(Lambertian::new(Color::new(1.0,0.0,0.0))) )));
    scene_graph.add_object(Box::new(Sphere::new(Transformation::new().scale(2.0,1.0,1.0).translate(Vector::new(2.0,0.0,4.0)),Box::new(Lambertian::new(Color::new(0.0,1.0,1.0))) )));
    //scene_graph.add_object(Box::new(Sphere::new(Transformation::new().translate(Vector::new(-2.0,0.0,4.0)),Box::new(Lambertian::new(Color::gray(0.50))) )));
    scene_graph.add_object(Box::new(Plane::new(Point::new(0.0,-1.0,0.0), Normal::new(0.0,1.0,0.0), false,Transformation::new(), Box::new(Lambertian::new(Color::gray(1.0))) )));
    scene_graph.add_object(Box::new(Triangle::new([Point::new(1.0,1.0,0.0),Point::new(-1.0,5.0,2.0),Point::new(-1.0,1.0,0.0)], true,Transformation::new(),Box::new(Lambertian::new(Color::gray(1.0))) )));
    scene_graph.add_object(Box::new(Rectangle::unit_square(Transformation::new().scale_all(4.0).rotate(RotationAxis::Xaxis, -FRAC_PI_2).translate(Vector::new(0.0,-1.0,6.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));
    //scene_graph.add_object(Box::new(BoxObject::new(Point::new(1.0,1.0,1.0), Transformation::new().translate(Vector::new(-4.0,2.0,4.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));
    let mesh = parse_obj("obj\\chair\\chair.obj").expect("Could not read obj");
    //let mesh = parse_obj("obj\\diamond.obj").expect("Could not read obj");
    //scene_graph.add_object(Box::new(mesh));

    //let position = math::Point::new(0.0,8.0,0.0);
    let position = math::Point::new(-2.0,2.0,0.0);
    let surface = Rectangle::unit_square(Transformation::new().rotate(RotationAxis::Xaxis, PI).translate(Vector::new(0.0,6.0,0.0)),  Box::new(Lambertian::new(Color::gray(1.0))) );
    scene_graph.add_light( Box::new(SurfaceLight::new(surface ,1000.0, Color::gray(1.0))) );
    scene_graph.add_light( Box::new(PointLight::new(position,200.0, Color::gray(1.0))) );
    //scene_graph.add_light( Box::new(PointLight::new(position,2000.0, Color::gray(1.0))) );
    scene_graph
}