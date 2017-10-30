extern crate rand;

use rand::distributions::{Exp, IndependentSample};

pub struct Generator {
    dist: Exp,
}

impl Generator {
    pub fn new(rate: f64) -> Generator {
        Generator {
            dist: Exp::new(rate),
        }
    }

    pub fn get_next(&mut self) -> f64 {
        self.dist.ind_sample(&mut rand::thread_rng())
    }
}
