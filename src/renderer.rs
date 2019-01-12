
extern crate sdl2;
extern crate itertools;

use self::itertools::iproduct;
use std::iter::Iterator;

use std::sync::{mpsc, Arc};
use std::sync::mpsc::{Sender};
use std::sync::atomic::{AtomicBool,Ordering};

use settings::Settings;
use camera::{Pixel, PerspectiveCamera};
use scene::{SceneGraph, Intersection};
use math::{Point};
use units::{Color, Radiance};

pub fn render(settings: Arc<Settings>, camera: PerspectiveCamera, scene: SceneGraph){
    let camera = Arc::new(camera);
    let scene = Arc::new(scene);
    run_program_loop(settings, camera, scene);
}

fn handle_threads(chunks: &mut Iterator<Item=(i32,i32)>, thread_pool: &mut Vec<Arc<AtomicBool>>, settings: &Arc<Settings>, camera: &Arc<PerspectiveCamera>, scene_graph: &Arc<SceneGraph>, sender : &Sender<Vec<(Pixel,Color)>>){
    let width = settings.screen_width;
    let height = settings.screen_height;
    let chunk_width = settings.chunk_width;
    let chunk_height = settings.chunk_height;

    for thread in thread_pool.iter_mut() {
        let is_ready = thread.load(Ordering::Relaxed);
        if is_ready {
            if let Some((h,w)) = chunks.next(){
                let sender_clone = Sender::clone(&sender);
                let settings = settings.clone();
                let scene = scene_graph.clone();
                let camera = camera.clone();
                let thread_is_ready = thread.clone();
                thread_is_ready.store(false, Ordering::Relaxed);

                std::thread::spawn( move || {
                    let mut pixels = vec![];
                    let h = h*chunk_height;
                    let till_h = if height-h < chunk_height {h+(height-h)} else {h+chunk_height};

                    for y in h..till_h{
                        let w = w*chunk_width;
                        let till_w = if width-w < chunk_width {w+(width-w)} else {w+chunk_width};

                        for x in w..till_w{
                            pixels.push(calucate_pixel(Pixel{x,y}, settings.aa_multi_sample, &camera, &scene));
                        }
                    }
                    sender_clone.send(pixels).unwrap();
                    thread_is_ready.store(true, Ordering::Relaxed);
                });
            }
        }
    }
}

fn calucate_pixel(pixel: Pixel, multi_sample: i32, camera: &PerspectiveCamera, scene: &SceneGraph) -> (Pixel, Color) {
    let mut intersect = None;
    let rad = camera.rays_for_pixel(&pixel).iter().map(|ray|{
        (scene.intersect(ray), ray.direction())
    }).map(|(int,dir)|{
        intersect = int.clone();
        match int {
            None => Radiance::zero(),
            Some(i) => scene.receive_radiance(i, dir.invert())
        }
    }).fold(Radiance::zero(), |acc,rad|{
        acc + rad
    }) * (1.0/(multi_sample*multi_sample) as f64);

    let color = radiance_color_map(rad);
    //let color = normal_color_map(intersect);
    //let color = distance_color_map(intersect, camera.position());
    (pixel,color)
}

fn radiance_color_map(rad: Radiance ) -> Color{
    Color::radiance_to_color(rad)
}

fn normal_color_map(intersect: Option<Intersection>) -> Color{
    match intersect {
        None => Color::black(),
        Some(intersect) => {
            let normal = (*intersect.normal().base()+1.0) / 2.0;
            Color::new(normal.x, normal.y, normal.z)
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

fn run_program_loop(settings: Arc<Settings>, camera: Arc<PerspectiveCamera>, scene: Arc<SceneGraph>){

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust renderer", settings.screen_width as u32, settings.screen_height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.present();

    let (sender, receiver) = mpsc::channel();
    let mut thread_pool = (0..settings.amt_threads as usize).map(|_| Arc::new(AtomicBool::new(true))).collect();
    let mut chunks = iproduct!(0..(settings.screen_height/settings.chunk_height)+1, 0..(settings.screen_width/settings.chunk_width)+1).peekable();

    let mut quit = false;
    let mut done = false;
    while !quit {
        if !done {
            handle_threads( &mut chunks, &mut thread_pool, &settings, &camera, &scene, &sender);

            while let Ok(pixels) = receiver.try_recv(){
                for (pixel,color) in pixels{
                    let r = (color.r()*255.0) as u8;
                    let g = (color.g()*255.0) as u8;
                    let b = (color.b()*255.0) as u8;
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(r,g,b));
                    canvas.draw_point(sdl2::rect::Point::new(pixel.x,pixel.y)).unwrap();
                }
                canvas.present();
            }

            if chunks.peek().is_none() && thread_pool.iter().fold(true,|acc, thread| acc && thread.load(Ordering::Relaxed)) {
                done = true;
                println!("done");
            }
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