
#[derive(Copy, Clone)]
pub enum Event {
    Arrival,
    Departure,
    Stop,
}

pub struct ScheduledEvent {
    pub event: Event,
    pub time: f64,
}

impl ScheduledEvent {
    pub fn new(event: Event, time: f64) -> ScheduledEvent {
        ScheduledEvent {
            event,
            time,
        }
    }
}
