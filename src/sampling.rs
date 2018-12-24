
extern crate rand;

pub enum SamplingTechnique {
    Grid,
    Random{seed: f64},
    Stratified{seed: f64}
}

pub fn sample_rect(width: f64, height: f64, technique: SamplingTechnique, sqrt_amt: u32) -> Vec<(f64,f64)> {
    match technique {
        SamplingTechnique::Grid => (0..sqrt_amt).zip(0..sqrt_amt).map(|(i,j)|{
            let u = (width/sqrt_amt as f64)*(i as f64 + 0.5);
            let v = (height/sqrt_amt as f64)*(j as f64 + 0.5);
            (u,v)
        }).collect(),
        SamplingTechnique::Random{..} => (0..(sqrt_amt*sqrt_amt)).map(|_| {
            let u = width*rand::random::<f64>();
            let v = height*rand::random::<f64>();
            (u,v)
        }).collect(),
        SamplingTechnique::Stratified{..} => (0..sqrt_amt).zip(0..sqrt_amt).map(|(i,j)|{
            let u = (width/sqrt_amt as f64)*(i as f64 + rand::random::<f64>());
            let v = (height/sqrt_amt as f64)*(j as f64 + rand::random::<f64>());
            (u,v)
        }).collect()
    }

}