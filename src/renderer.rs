
extern crate sdl2;
extern crate itertools;

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::SurfaceCanvas;
use sdl2::surface::Surface;

use self::itertools::iproduct;
use std::iter::Iterator;
use std::time::{Instant};

use std::sync::{mpsc, Arc};
use std::sync::mpsc::{Sender};

use crate::settings;
use crate::thread_pool::{ThreadPool};
use crate::camera::{Pixel, PerspectiveCamera};
use crate::scene::{Scene, Intersection};
use crate::math::{Point};
use crate::cg_tools::{Color, Radiance};

pub fn render(scene: Scene){
    let scene = Arc::new(scene);
    run_program_loop(scene);
}

#[derive(Debug)]
enum ChunkFinished {
    Chunk(Vec<(Pixel,Color)>),
    Done
}

fn init_threads(chunks: &mut dyn Iterator<Item=(u32,u32)>, thread_pool: &ThreadPool, scene_graph: &Arc<Scene>, sender : &Sender<ChunkFinished>){
    let settings = settings::get();
    let width = settings.screen_width;
    let height = settings.screen_height;
    let chunk_width = settings.chunk_width;
    let chunk_height = settings.chunk_height;

    for (h,w) in chunks {
        let sender_clone = Sender::clone(&sender);
        let scene = scene_graph.clone();

        thread_pool.execute( move || {
            let mut pixels = Vec::new();
            let h = h*chunk_height;
            let till_h = if height-h < chunk_height {h+(height-h)} else {h+chunk_height};

            for y in h..till_h{
                let w = w*chunk_width;
                let till_w = if width-w < chunk_width {w+(width-w)} else {w+chunk_width};

                for x in w..till_w{
                    pixels.push(calucate_pixel(Pixel{x: x as i32,y: y as i32}, &scene));
                }
            }
            sender_clone.send(ChunkFinished::Chunk(pixels)).unwrap();
        });
    }

    for _ in 0..thread_pool.amt_threads() {
        let sender_clone = Sender::clone(&sender);
        thread_pool.finish( move || {
            sender_clone.send(ChunkFinished::Done).unwrap();
        });
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RenderMode {
    Default,
    Normals,
    Distance(f64),
    BoundingBox
}

fn calucate_pixel(pixel: Pixel, scene: &Scene) -> (Pixel, Color) {
    let settings = settings::get();
    let mut intersect = None;
    let rad = scene.camera().rays_for_pixel(&pixel).iter().map(|ray|{
        (scene.intersect(ray), ray.direction())
    }).map(|(int,dir)|{
        intersect = int.clone();
        match int {
            None => Radiance::zero(),
            Some(i) => scene.receive_radiance(i, dir.invert())
        }
    }).fold(Radiance::zero(), |acc,rad|{
        acc + rad
    }) * (1.0/(settings.aa_multi_sample.pow(2)) as f64);

    let color = match settings.render_mode {
        RenderMode::Default => radiance_color_map(rad, settings.gamma),
        RenderMode::Normals => normal_color_map(intersect),
        RenderMode::Distance(factor) => distance_color_map(intersect, scene.camera().position(), factor),
        RenderMode::BoundingBox => normal_color_map(intersect),
    };
    (pixel,color)
}

fn radiance_color_map(rad: Radiance, gamma: f64) -> Color{
    let color = Color::from(rad);
    color.gamma_correct(gamma).clamped_rgb()
}

fn normal_color_map(intersect: Option<Intersection>) -> Color{
    match intersect {
        None => Color::black(),
        Some(intersect) => {
            let normal = (**intersect.normal()+1.0) / 2.0;
            Color::new_rgb(normal.x, normal.y, normal.z)
        }
    }
}

fn distance_color_map(intersect: Option<Intersection>, camera_position: Point, factor: f64) -> Color{
    match intersect {
        None => Color::black(),
        Some(intersect) => {
            //let distance = (intersect.point() - camera_position).length() / factor;
            let distance = intersect.t() / factor;
            Color::gray_scale(1.0/distance)
        }
    }
}

fn run_program_loop(scene: Arc<Scene>){
    let settings = settings::get();
    let sdl_context = sdl2::init().unwrap();

    let surface = Surface::new(settings.screen_width as u32, settings.screen_height as u32, PixelFormatEnum::RGB888).unwrap();
    let mut canvas = surface.into_canvas().unwrap();

    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem.window("rust renderer", settings.screen_width as u32, settings.screen_height as u32)
    //     .position_centered()
    //     .opengl()
    //     .build()
    //     .unwrap();
    // let mut canvas = window.into_canvas().build().unwrap();

    canvas.present();

    //START
    let now = Instant::now();

    let (sender, receiver) = mpsc::channel();
    let mut thread_pool = ThreadPool::new(settings.amt_threads);
    let mut chunks = iproduct!(0..(settings.screen_height/settings.chunk_height)+1, 0..(settings.screen_width/settings.chunk_width)+1).peekable();
    init_threads(&mut chunks, &mut thread_pool,&scene, &sender);
    thread_pool.finish_jobs();

    let mut quit = false;
    let mut finished_jobs = 0;
    while !quit {
        if finished_jobs < settings.amt_threads {
            while let Ok(chunk_finished) = receiver.recv() {
                match chunk_finished {
                    ChunkFinished::Done => {
                        finished_jobs += 1;
                        if finished_jobs == settings.amt_threads {
                            let elapsed = now.elapsed();
                            println!("Done.");
                            println!("Elapsed time {}.{}s", elapsed.as_secs(), elapsed.subsec_millis());
                            super::statistics::print_statistics();
                            quit = true;
                            break;
                        }
                    },
                    ChunkFinished::Chunk(pixels) => {
                        for (pixel,color) in pixels{
                            let (r,g,b) = color.rgb();
                            let r = (r*255.0) as u8;
                            let g = (g*255.0) as u8;
                            let b = (b*255.0) as u8;
                            canvas.set_draw_color(sdl2::pixels::Color::RGB(r,g,b));
                            canvas.draw_point(sdl2::rect::Point::new(pixel.x,pixel.y)).unwrap();
                        }
                        canvas.present();
                    }
                }
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

    std::fs::create_dir_all("img").unwrap();
    canvas.surface().save_bmp("img/result_img.bmp").unwrap();
}