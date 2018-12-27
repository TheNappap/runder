
mod renderer;
mod math;
mod primitives;
mod camera;
mod lights;
mod scene;
mod material;
mod sampling;
mod settings;

fn main() {
    let settings = settings::Settings {
        screen_width: 800,
        screen_height: 600,
        aa_multi_sample: 2,
        sampling_technique: sampling::SamplingTechnique::Stratified{multi_sample: 2, seed: 0.0}
    };

    renderer::render(settings);
}