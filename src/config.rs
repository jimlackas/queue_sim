extern crate clap;

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
        let matches = clap_app!(queue_sim =>
            (version: "1.0")
            (author: "Jim Lackas")
            (about: "Simulates a simple MM1 queue.")
            (@arg ARRIVAL_RATE: -a --arrival +takes_value "Sets the inter-arrival rate.")
            (@arg SERVICE_RATE: -s --service +takes_value "Sets the service rate.")
            (@arg TIME: -t --time +takes_value "Sets the amount of time to simulate.")
        ).get_matches();

        println!("*******************************************************");
        println!("Configuration values:");
        let arrival_rate = parse_arg_value(matches.value_of("ARRIVAL_RATE"), INTER_ARRIVAL_RATE);
        println!("   Arrival rate = {:.3}", arrival_rate);

        let service_rate = parse_arg_value(matches.value_of("SERVICE_RATE"), SERVICE_RATE);
        println!("   Service rate = {:.3}", service_rate);

        let sim_time = parse_arg_value(matches.value_of("TIME"), SIM_TIME);
        println!("   Time to simulate = {:.1}", sim_time);
        println!();

        Config {
            arrival_rate,
            service_rate,
            sim_time,
        }
    }
}

fn parse_arg_value(arg: Option<&str>, default: f64) -> f64 {
    match arg {
        Some(arg_val) => {
            match arg_val.parse::<f64>() {
                Ok(val) => val,
                Err(_) => default,
            }
        }
        None => default,
    }
}