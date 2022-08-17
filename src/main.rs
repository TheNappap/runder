
mod acceleration;
mod renderer;
mod math;
mod cg_tools;
mod camera;
mod objects;
mod scene;
mod settings;
mod statistics;
mod thread_pool;

use std::f64::consts::{PI,FRAC_PI_2,FRAC_PI_4};
use std::sync::Arc;

use objects::*;
use camera::{PerspectiveCamera};
use scene::{Scene};
use math::{Point, Vector, Normal, Direction, RotationAxis};
use cg_tools::{Transformation,Color,WhiteReference};


fn main() {
    let settings = settings::Settings{
        render_mode: settings::RenderMode::Default,
        gamma: 2.2,
        color_model: settings::ColorModel::RGB,
        acceleration_structure: settings::AccelerationStructureKind::BVH,
        amt_threads: 6,
        aa_multi_sample: 4,
        light_sampling_technique: settings::SamplingTechnique::Stratified{multi_sample: 4, seed: 0.0},
        ..settings::DEFAULT_SETTINGS
    };

    settings::set(settings);
    let scene = default_scene();

    renderer::render(scene);
}

fn default_camera() -> PerspectiveCamera
{
    let position = Point::new(0.,0.5,-3.0);
    let direction = Direction::new(0.0,0.0,1.);
    let up = Direction::up();
    PerspectiveCamera::new(position,direction,up,60.)
}

fn default_scene() -> Scene {
    let mut instances: Vec<Instance> = Vec::new();

    let sphere1 = Arc::new( Sphere::new(Box::new(Lambertian::new(Color::new_rgb(1.,0.,0.))) ) );
    instances.push(Instance::transformed(sphere1, Transformation::new()
        .translate(Vector::new(0.,0.,3.))));
    let sphere2 = Arc::new( Sphere::new(Box::new(Lambertian::new(Color::new_rgb(0.,1.,1.))) ) );
    instances.push(Instance::transformed(sphere2, Transformation::new()
        .scale(2.,1.,1.)
        .rotate(RotationAxis::Zaxis, FRAC_PI_4)
        .translate(Vector::new(2.,0.,4.))));
    //let sphere3 = Arc::new( Sphere::new(Box::new(Lambertian::new(Color::new_rgb(0.,1.,0.))) ) );
    //instances.push(Instance::transformed( sphere3,Transformation::new()
    //    .translate(Vector::new(-2.,0.,4.))));

    let plane = Arc::new( Plane::new(Point::new(0.,-1.,0.), Normal::new(0.,1.,0.), false, Box::new(Lambertian::new(Color::gray_scale(1.))) ) );
    instances.push(Instance::new(plane));
    //let triangle = Arc::new( Triangle::new([Point::new(-1.,0.,2.),Point::new(-1.,1.,5.),Point::new(3.,0.,2.)], false,Box::new(Lambertian::new(Color::gray_scale(1.))) ) );
    //instances.push(Instance::new(triangle));
    //let rectangle= Arc::new( Rectangle::unit_square(true,Box::new(Lambertian::new(Color::gray_scale(1.))) ) );
    //instances.push(Instance::transformed(rectangle,Transformation::new()
    //    .scale_all(4.)
    //    .rotate(RotationAxis::Xaxis, -FRAC_PI_2)
    //    .translate(Vector::new(0.,-1.,6.))));
    //let box_object = Arc::new( BoxObject::new_from_origin(Point::new(1.,1.,1.),Box::new(Lambertian::new(Color::gray_scale(1.))) ) );
    //instances.push(Instance::transformed(box_object,Transformation::new()
    //    .translate(Vector::new(-4.,2.,4.))));

    let transformation = Transformation::new().translate(Vector::new(-2.0,0.0,7.0));
    let chair_mesh = Arc::new( parse_obj("obj\\chair\\chair.obj").expect("Could not read obj") );
    let diamond_mesh = Arc::new( parse_obj("obj\\diamond.obj").expect("Could not read obj") );
    instances.push(Instance::transformed(chair_mesh, transformation.clone()));
    instances.push(Instance::transformed(diamond_mesh, transformation));

    let mut lights: Vec<Box<dyn Light>> = Vec::new();
    let position = math::Point::new(-2.,2.,0.);
    //let position2 = math::Point::new(0.,8.,0.);
    let surface = Rectangle::unit_square(false, Box::new(Lambertian::new(Color::gray_scale(1.))) );
    let transformation = Transformation::new()
        .rotate(RotationAxis::Xaxis, PI)
        .translate(Vector::new(0.,6.,0.));
    lights.push( Box::new(SurfaceLight::transformed(surface, transformation,1000., Color::white(WhiteReference::E))) );
    lights.push( Box::new(PointLight::new(position,600., Color::white(WhiteReference::E))) );
    //lights.push( Box::new(PointLight::new(position2,2000., Color::gray_scale(1.))) );

    Scene::new(instances, lights, default_camera())
}