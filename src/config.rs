
const INTER_ARRIVAL_RATE: f64 = 1.0 / 0.9;
const SERVICE_RATE: f64 = 1.0 / 0.7;
const SIM_TIME: f64 = 100_000.0;

pub struct Config {
    pub arrival_rate: f64,
    pub service_rate: f64,
    pub sim_time: f64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            arrival_rate: INTER_ARRIVAL_RATE,
            service_rate: SERVICE_RATE,
            sim_time: SIM_TIME,
        }
    }
}