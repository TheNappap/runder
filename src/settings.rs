
use sampling::SamplingTechnique;

#[derive(Clone)]
pub struct Settings{
    pub screen_width: i32,
    pub screen_height: i32,
    pub aa_multi_sample: i32,
    pub light_sampling_technique: SamplingTechnique
}