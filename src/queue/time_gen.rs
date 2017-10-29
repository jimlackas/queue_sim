extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::Read;

pub enum GeneratorInput<'a> {
    File(&'a str),
    Generated(usize),
}

pub struct Generator {
    time_values : Vec<f64>,
}

impl Generator {
    pub fn new(rate: f64, input: GeneratorInput) -> Generator {
        let mut time_values = Vec::new();
        match input {
            GeneratorInput::File(filename) => {
                if let Ok(mut input_file) = File::open(filename) {

                    let mut contents = String::new();
                    match input_file.read_to_string(&mut contents) {
                        Ok(_) => {
                            time_values =
                                contents.lines()
                                    .map(|u| {
                                        match u.parse::<f64>() {
                                            Ok(u) => -((1.0 - u).ln())/rate,
                                            Err(_) => 0.0
                                        }
                                    })
                                    .collect();
                        },
                        Err(_) => ()
                    }

                }
            }

            GeneratorInput::Generated(num) => {
                let mut rng = rand::thread_rng();
                let mut uniform_randoms: Vec<f64> = Vec::with_capacity(num);
                for _ in 0..num {
                    uniform_randoms.push(rng.gen_range(0.0, 1.0));
                }
                time_values =
                    uniform_randoms.iter()
                        .map(|u| -((1.0 - u).ln())/rate)
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