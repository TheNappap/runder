
extern crate sdl2;

use std::sync::{mpsc, Arc};
use std::sync::mpsc::{Sender, Receiver};
use std::f64::consts::{PI,FRAC_PI_2};

use primitives::{Sphere,Plane,Rectangle,BoxObject};
use camera::{Pixel, PerspectiveCamera};
use scene::{SceneGraph, Intersection};
use lights::{PointLight, SurfaceLight};
use math::{VectorTrait, Point, Normal, Vector, Transformation, Ray, RotationAxis};
use material::{Color,Lambertian};

fn default_camera() -> PerspectiveCamera
{
    let position = super::math::Point::origin();
    let direction = super::math::Direction::new(0.0,0.0,1.0);
    let up = super::math::Direction::new(0.0,1.0,0.0);
    PerspectiveCamera::new(position,direction,up,80.0)
}

fn default_scene() -> SceneGraph{
    let mut scene_graph = SceneGraph::new();
    scene_graph.add_object(Box::new(Sphere::new(Transformation::new().translate(Vector::new(0.0, 0.0, 2.0)), Box::new(Lambertian::new(Color::new(1.0,0.0, 0.0))) )));
    scene_graph.add_object(Box::new(Sphere::new(Transformation::new().scale(2.0, 1.0, 1.0).translate(Vector::new(2.0, 0.0, 3.0)),Box::new(Lambertian::new(Color::new(0.0,1.0, 1.0))) )));
    scene_graph.add_object(Box::new(Sphere::new(Transformation::new().translate(Vector::new(-2.0, 0.0, 3.0)),Box::new(Lambertian::new(Color::gray(0.50))) )));
    scene_graph.add_object(Box::new(Plane::new(Transformation::new().translate(Vector::new(0.0, -1.0, 0.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));
    scene_graph.add_object(Box::new(Rectangle::new(Point::new(2.0, 0.0, 2.0), Transformation::new().rotate(RotationAxis::Xaxis, -FRAC_PI_2).translate(Vector::new(0.0, 0.0, 3.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));
    scene_graph.add_object(Box::new(BoxObject::new(Point::new(1.0, 1.0, 1.0), Transformation::new().translate(Vector::new(-4.0, 2.0, 3.0)), Box::new(Lambertian::new(Color::gray(1.0))) )));

    let position = super::math::Point::new(0.0,8.0,0.0);
    let position2 = super::math::Point::new(-2.0,2.0,0.0);
    //scene_graph.add_light( Box::new(PointLight::new(position,2000.0, Color::gray(1.0))) );
    let surface = Rectangle::new(Point::new(2.0, 0.0, 2.0), Transformation::new().rotate(RotationAxis::Xaxis, PI).translate(Vector::new(0.0, 6.0, 0.0)), Box::new(Lambertian::new(Color::gray(1.0))) );
    scene_graph.add_light( Box::new(SurfaceLight::new(surface ,1000.0, Color::gray(1.0))) );
    scene_graph.add_light( Box::new(PointLight::new(position2,200.0, Color::gray(1.0))) );
    scene_graph
}

pub fn render(width : i32, height : i32){
    run_program_loop(width, height);
}

fn start_threads(sender : Sender<Vec<(Pixel,Color)>>, width : i32, height : i32) {
    let chunk_width = 80;
    let chunk_height = 60;
    let camera = default_camera();
    let scene_graph = Arc::new(default_scene());

    for w in 0..(width/chunk_width)+1{
        for h in 0..(height/chunk_height)+1{
            let sender_clone = Sender::clone(&sender);
            let scene = scene_graph.clone();

            std::thread::spawn( move || {
                let mut pixels = vec![];
                let w = w*chunk_width;
                let till_w = if width-w < chunk_width {w+(width-w)} else {w+chunk_width};

                for x in w..till_w{
                    let h = h*chunk_height;
                    let till_h = if height-h < chunk_height {h+(height-h)} else {h+chunk_height};

                    for y in h..till_h{
                        let ray = camera.ray_for_pixel(width, height, Pixel{x,y});
                        let intersect = scene.intersect(&ray);
                        let color = radiance_color_map(&scene, intersect, ray);
                        //let color = normal_color_map(intersect);
                        //let color = distance_color_map(intersect, camera.position());
                        pixels.push((Pixel{x,y},color));
                    }
                }
                sender_clone.send(pixels).unwrap();
            });
        }
    }
}

fn radiance_color_map(scene: &SceneGraph , intersect: Option<Intersection>, ray: Ray ) -> Color{
    match intersect {
        None => Color::black(),
        Some(intersect) => Color::radiance_to_color(scene.receive_radiance(intersect, ray.direction().invert()))
    }
}

fn normal_color_map(intersect: Option<Intersection>) -> Color{
    match intersect {
        None => Color::black(),
        Some(intersect) => {
            let normal = (*intersect.normal().base()+1.0) / 2.0;
            Color::new(normal.x(), normal.y(), normal.z())
        }
    }
}

fn distance_color_map(intersect: Option<Intersection>, camera_position: Point) -> Color{
    match intersect {
        None => Color::black(),
        Some(intersect) => {
            let distance = (intersect.point() - camera_position).length();
            Color::gray(1.0/distance)
        }
    }
}

fn run_program_loop(width : i32, height : i32){

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust renderer", width as u32, height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.present();

    let (sender, receiver) = mpsc::channel();
    std::thread::spawn( move || {
        start_threads(sender, width, height);
    });

    let mut quit = false;
    while !quit {
        if let Ok(pixels) = receiver.try_recv(){
            for (pixel,color) in pixels{
                let r = (color.r()*255.0) as u8;
                let g = (color.g()*255.0) as u8;
                let b = (color.b()*255.0) as u8;
                canvas.set_draw_color(sdl2::pixels::Color::RGB(r,g,b));
                canvas.draw_point(sdl2::rect::Point::new(pixel.x,pixel.y)).unwrap();
            }
            canvas.present();
        }

        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut it = event_pump.poll_iter();
        while let Some(e) = it.next() {
            match e {
                sdl2::event::Event::Quit{timestamp:_} => quit = true,
                _ => ()
            }
        }
    }
}