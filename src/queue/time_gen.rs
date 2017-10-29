extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[allow(dead_code)]
pub enum GeneratorInput<'a> {
    File(&'a str),
    List(usize),
}

pub struct Generator {
    time_values : Vec<f64>,
}

impl Generator {
    pub fn new(rate: f64, input: GeneratorInput) -> Generator {
        let mut time_values = Vec::new();
        match input {
            GeneratorInput::File(filename) => {
                if let Ok(input_values) = read_values_from_file(filename) {
                    time_values =input_values.iter()
                        .map(|u| uniform_to_exp(u, &rate))
                        .collect();
                }
            }

            GeneratorInput::List(num) => {
                let mut rng = rand::thread_rng();
                let mut uniform_randoms: Vec<f64> = Vec::with_capacity(num);
                for _ in 0..num {
                    uniform_randoms.push(rng.gen_range(0.0, 1.0));
                }
                time_values = uniform_randoms.iter()
                    .map(|u| uniform_to_exp(u, &rate))
                    .collect();
            }
        }

        Generator {
            time_values,
        }
    }

    pub fn get_next(&mut self) -> Option<f64> {
        if self.time_values.len() > 0 {
            Some(self.time_values.remove(0))
        } else {
            None
        }

    }
}

fn read_values_from_file(filename: &str) -> Result<(Vec<f64>), Box<Error>> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let time_values = contents.lines()
        .map(|u| {
            match u.parse::<f64>() {
                Ok(u) => u,
                Err(_) => 0.0
            }
        })
        .collect();

    Ok((time_values))
}

fn uniform_to_exp(u: &f64, rate: &f64) -> f64 {
    -((1.0 - u).ln())/rate
}