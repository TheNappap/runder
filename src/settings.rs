
pub use crate::cg_tools::{SamplingTechnique,ColorModel};
pub use crate::acceleration::AccelerationStructureKind;
pub use crate::renderer::RenderMode;

#[derive(Clone)]
pub struct Settings{
    pub screen_width: u32,
    pub screen_height: u32,
    pub chunk_width: u32,
    pub chunk_height: u32,
    pub render_mode: RenderMode,
    pub gamma: f64,
    pub color_model: ColorModel,
    pub acceleration_structure: AccelerationStructureKind,
    pub amt_threads: usize,
    pub aa_multi_sample: u32,
    pub light_sampling_technique: SamplingTechnique
}

pub fn get() -> &'static Settings {
    unsafe{ &SETTINGS }
}

pub fn set(new_settings: Settings) {
    unsafe{ SETTINGS = new_settings; }
}

pub const DEFAULT_SETTINGS: Settings = Settings {
    screen_width: 800,
    screen_height: 600,
    chunk_width: 80,
    chunk_height: 60,
    render_mode: RenderMode::Default,
    gamma: 2.2,
    color_model: ColorModel::RGB,
    acceleration_structure: AccelerationStructureKind::BVH,
    amt_threads: 4,
    aa_multi_sample: 1,
    light_sampling_technique: SamplingTechnique::Stratified{multi_sample: 1, seed: 0.0}
};

static mut SETTINGS: Settings = DEFAULT_SETTINGS;