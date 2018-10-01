use event::*;

pub trait SimulatedSystem {
    fn handle_event(&mut self, event: &Event, time: f64) -> Vec<ScheduledEvent>;
    fn print_results(&self);
}