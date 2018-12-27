
extern crate rand;
extern crate itertools;
use self::itertools::iproduct;

#[derive(Copy, Clone)]
pub enum SamplingTechnique {
    Grid(i32),
    Random{
        multi_sample: i32,
        seed: f64
    },
    Stratified{
        multi_sample: i32,
        seed: f64
    }
}

impl SamplingTechnique {
    pub fn multi_sample(&self) -> i32 {
        match self {
            SamplingTechnique::Grid(ms) => *ms,
            SamplingTechnique::Random{multi_sample, ..} => *multi_sample,
            SamplingTechnique::Stratified{multi_sample, ..} => *multi_sample,
        }
    }
}

pub fn sample_rect(width: f64, height: f64, technique: SamplingTechnique) -> Vec<(f64,f64)> {
    let sqrt_amt = technique.multi_sample();
    match technique {
        SamplingTechnique::Grid(_) => iproduct!(0..sqrt_amt,0..sqrt_amt).map(|(i,j)|{
            let u = (width/sqrt_amt as f64)*(i as f64 + 0.5);
            let v = (height/sqrt_amt as f64)*(j as f64 + 0.5);
            (u,v)
        }).collect(),
        SamplingTechnique::Random{..} => (0..(sqrt_amt*sqrt_amt)).map(|_| {
            let u = width*rand::random::<f64>();
            let v = height*rand::random::<f64>();
            (u,v)
        }).collect(),
        SamplingTechnique::Stratified{..} => iproduct!(0..sqrt_amt,0..sqrt_amt).map(|(i,j)|{
            let u = (width/sqrt_amt as f64)*(i as f64 + rand::random::<f64>());
            let v = (height/sqrt_amt as f64)*(j as f64 + rand::random::<f64>());
            (u,v)
        }).collect()
    }

}