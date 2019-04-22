
mod acceleration;
mod renderer;
mod math;
mod cg_tools;
mod camera;
mod objects;
mod scene;
mod settings;
mod thread_pool;

use std::sync::{Arc};
use std::f64::consts::{PI,FRAC_PI_2,FRAC_PI_4};

use objects::*;
use camera::{PerspectiveCamera};
use scene::{SceneGraph};
use math::{Point, Vector, Normal, Direction, RotationAxis};
use cg_tools::{SamplingTechnique,Transformation,Color};
use acceleration::*;


fn main() {
    let settings = settings::Settings{
        gamma: 2.2,
        color_model: settings::ColorModel::RGB,
        amt_threads: 6,
        aa_multi_sample: 1,
        light_sampling_technique: SamplingTechnique::Stratified{multi_sample: 1, seed: 0.0},
        ..settings::DEFAULT_SETTINGS
    };

    settings::set(settings);
    let camera = default_camera();
    let scene = default_scene();

    renderer::render(camera, scene);
}

fn default_camera() -> PerspectiveCamera
{
    let position = Point::new(0.0,0.5,-3.0);
    let direction = Direction::new(0.0,0.0,1.0);
    let up = Direction::up();
    PerspectiveCamera::new(position,direction,up,60.0)
}

fn default_scene() -> SceneGraph{
    let mut objects: Vec<Box<Object>> = Vec::new();
    objects.push(Box::new(Sphere::new(Transformation::new().translate(Vector::new(0.0,0.0,3.0)), Box::new(Lambertian::new(Color::new(1.0,0.0,0.0))) )));
    objects.push(Box::new(Sphere::new(Transformation::new().scale(2.0,1.0,1.0).rotate(RotationAxis::Zaxis, FRAC_PI_4).translate(Vector::new(2.0,0.0,4.0)),Box::new(Lambertian::new(Color::new(0.0,1.0,1.0))) )));
    objects.push(Box::new(Sphere::new(Transformation::new().translate(Vector::new(-2.0,0.0,4.0)),Box::new(Lambertian::new(Color::gray(0.50))) )));
    objects.push(Box::new(Plane::new(Point::new(0.0,-1.0,0.0), Normal::new(0.0,1.0,0.0), false,Transformation::new(), Box::new(Lambertian::new(Color::gray(1.0))) )));
    //objects.push(Box::new(Triangle::new([Point::new(-1.0,0.0,2.0),Point::new(-1.0,1.0,5.0),Point::new(3.0,0.0,2.0)], false,Transformation::new(),Box::new(Lambertian::new(Color::gray(1.0))) )));
    objects.push(Box::new(Rectangle::unit_square(Transformation::new().scale_all(4.0).rotate(RotationAxis::Xaxis, -FRAC_PI_2).translate(Vector::new(0.0,-1.0,6.0)), true,Box::new(Lambertian::new(Color::gray(1.0))) )));
    objects.push(Box::new(BoxObject::new_from_origin(Point::new(1.0,1.0,1.0), Transformation::new().translate(Vector::new(-4.0,2.0,4.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));
    //let mesh = parse_obj("obj\\chair\\chair.obj").expect("Could not read obj");
    let mesh = parse_obj("obj\\diamond.obj").expect("Could not read obj");
    objects.push(Box::new(mesh));
    let acc_structure = Box::new(BoundingVolumeHierarchy::new(objects));

    let mut lights: Vec<Box<Light>> = Vec::new();
    //let position = math::Point::new(0.0,8.0,0.0);
    let position = math::Point::new(-2.0,2.0,0.0);
    let surface = Rectangle::unit_square(Transformation::new().rotate(RotationAxis::Xaxis, PI).translate(Vector::new(0.0,6.0,0.0)), false, Box::new(Lambertian::new(Color::gray(1.0))) );
    lights.push( Box::new(SurfaceLight::new(surface ,1000.0, Color::gray(1.0))) );
    lights.push( Box::new(PointLight::new(position,200.0, Color::gray(1.0))) );
    //lights.push( Box::new(PointLight::new(position,2000.0, Color::gray(1.0))) );

    SceneGraph::new(acc_structure, lights)
}