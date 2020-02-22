
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

    pub fn sample_rect(&self, width: f64, height: f64) -> Vec<(f64,f64)> {
        let sqrt_amt = self.multi_sample();
        match self {
            SamplingTechnique::Grid(_) => iproduct!(0..sqrt_amt,0..sqrt_amt).map(|(i,j)|{
                let u = (width/sqrt_amt as f64)*(i as f64 + 0.5);
                let v = (height/sqrt_amt as f64)*(j as f64 + 0.5);
                (u,v)
            }).collect(),
            SamplingTechnique::Random{..} => self.sample_rect_random(width, height, sqrt_amt*sqrt_amt),
            SamplingTechnique::Stratified{..} => iproduct!(0..sqrt_amt,0..sqrt_amt).map(|(i,j)|{
                let u = (width/sqrt_amt as f64)*(i as f64 + rand::random::<f64>());
                let v = (height/sqrt_amt as f64)*(j as f64 + rand::random::<f64>());
                (u,v)
            }).collect()
        }
    }

    fn sample_rect_random(&self, width: f64, height: f64, sample_amt: i32) -> Vec<(f64,f64)> {
        (0..sample_amt).map(|_| {
            let u = width*rand::random::<f64>();
            let v = height*rand::random::<f64>();
            (u,v)
        }).collect()
    }

    pub fn sample_hemisphere(&self, branching_factor: i32) -> Vec<(f64,f64,f64)> {
        //https://cg.informatik.uni-freiburg.de/course_notes/graphics2_08_renderingEquation.pdf
        self.sample_rect_random(1., 1., branching_factor).iter().map(|(u,v)| {
            let cos_theta = (1. - u).sqrt();
            let sin_theta = u.sqrt();
            let phi = 2. * std::f64::consts::PI * v;

            let x = sin_theta * phi.cos();
            let z = sin_theta * phi.sin();
            (x,cos_theta,z)
        }).collect()
    }
}
