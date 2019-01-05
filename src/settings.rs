
use cg_tools::SamplingTechnique;

#[derive(Clone)]
pub struct Settings{
    pub screen_width: i32,
    pub screen_height: i32,
    pub chunk_width: i32,
    pub chunk_height: i32,
    pub amt_threads: i32,
    pub aa_multi_sample: i32,
    pub light_sampling_technique: SamplingTechnique
}