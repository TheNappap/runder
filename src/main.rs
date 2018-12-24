
mod renderer;
mod math;
mod primitives;
mod camera;
mod lights;
mod scene;
mod material;
mod sampling;

fn main() {
    let width = 800;
    let height = 600;

    renderer::render(width, height);
}