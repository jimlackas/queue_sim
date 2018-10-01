extern crate rand;
#[macro_use]
extern crate clap;

mod event;
mod queue;
mod schedule;
mod config;
mod system;

use queue::Queue;
use schedule::Schedule;
use event::{Event, ScheduledEvent};
use config::Config;


fn main() {
    let config = Config::new();
    let queue = Queue::new(config.arrival_rate, config.service_rate);
    let mut schedule = Schedule::<Queue>::new(queue);
    schedule.schedule_event(ScheduledEvent::new(Event::Arrival, 0.0));
    schedule.schedule_event(ScheduledEvent::new(Event::Stop, config.sim_time));
    schedule.run();
}
