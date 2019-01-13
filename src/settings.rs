
use cg_tools::SamplingTechnique;

#[derive(Clone)]
pub struct Settings{
    pub screen_width: u32,
    pub screen_height: u32,
    pub chunk_width: u32,
    pub chunk_height: u32,
    pub amt_threads: usize,
    pub aa_multi_sample: u32,
    pub light_sampling_technique: SamplingTechnique
}