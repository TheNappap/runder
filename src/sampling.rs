
extern crate rand;

pub enum SamplingTechnique {
    Grid,
    Random{seed: f64},
    Stratified{seed: f64}
}

pub fn sample_rect(width: f64, height: f64) -> (f64,f64) {
    let u = width*rand::random::<f64>();
    let v = height*rand::random::<f64>();
    (u,v)
}