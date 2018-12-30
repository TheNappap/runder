
mod renderer;
mod math;
mod primitives;
mod camera;
mod lights;
mod scene;
mod material;
mod sampling;
mod settings;

use std::sync::{Arc};
use std::f64::consts::{PI,FRAC_PI_2};

use settings::Settings;
use primitives::{Sphere,Plane,Rectangle,BoxObject,Triangle};
use camera::{PerspectiveCamera};
use scene::{SceneGraph};
use lights::{PointLight, SurfaceLight};
use math::{Point, Vector, Transformation, RotationAxis};
use material::{Color,Lambertian};


fn main() {
    let settings = settings::Settings {
        screen_width: 800,
        screen_height: 600,
        aa_multi_sample: 4,
        light_sampling_technique: sampling::SamplingTechnique::Stratified{multi_sample: 2, seed: 0.0}
    };

    let settings = Arc::new(settings);
    let camera = default_camera(settings.clone());
    let scene = default_scene(settings.clone());

    renderer::render(settings, camera, scene);
}

fn default_camera(settings: Arc<Settings>) -> PerspectiveCamera
{
    let position = math::Point::origin();
    let direction = math::Direction::new(0.0,0.0,1.0);
    let up = math::Direction::new(0.0,1.0,0.0);
    PerspectiveCamera::new(settings, position,direction,up,80.0)
}

fn default_scene(settings: Arc<Settings>) -> SceneGraph{
    let mut scene_graph = SceneGraph::new(settings);
    scene_graph.add_object(Box::new(Sphere::new(Transformation::new().translate(Vector::new(0.0, 0.0, -3.0)), Box::new(Lambertian::new(Color::new(1.0,0.0, 0.0))) )));
    scene_graph.add_object(Box::new(Sphere::new(Transformation::new().scale(2.0, 1.0, 1.0).translate(Vector::new(2.0, 0.0, -4.0)),Box::new(Lambertian::new(Color::new(0.0,1.0, 1.0))) )));
    scene_graph.add_object(Box::new(Sphere::new(Transformation::new().translate(Vector::new(-2.0, 0.0, -4.0)),Box::new(Lambertian::new(Color::gray(0.50))) )));
    scene_graph.add_object(Box::new(Plane::new(Transformation::new().translate(Vector::new(0.0, -1.0, 0.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));
    scene_graph.add_object(Box::new(Rectangle::new(Point::new(4.0, 0.0, 4.0), Transformation::new().rotate(RotationAxis::Xaxis, FRAC_PI_2).translate(Vector::new(0.0, 2.0, -4.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));
    scene_graph.add_object(Box::new(BoxObject::new(Point::new(1.0, 1.0, 1.0), Transformation::new().translate(Vector::new(-4.0, 2.0, -4.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));
    scene_graph.add_object(Box::new(Triangle::new(Point::new(1.0, -0.5, -2.0),Point::new(-1.0, -0.5, -1.0),Point::new(1.0, -0.5, -1.0), Transformation::new(), Box::new(Lambertian::new(Color::gray(1.0))) )));

    //let position = math::Point::new(0.0,8.0,0.0);
    let position = math::Point::new(-2.0,2.0,0.0);
    let surface = Rectangle::new(Point::new(2.0, 0.0, 2.0), Transformation::new().rotate(RotationAxis::Xaxis, PI).translate(Vector::new(0.0, 6.0, 0.0)), Box::new(Lambertian::new(Color::gray(1.0))) );
    scene_graph.add_light( Box::new(SurfaceLight::new(surface ,1000.0, Color::gray(1.0))) );
    scene_graph.add_light( Box::new(PointLight::new(position,200.0, Color::gray(1.0))) );
    //scene_graph.add_light( Box::new(PointLight::new(position,2000.0, Color::gray(1.0))) );
    scene_graph
}