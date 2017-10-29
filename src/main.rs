extern crate rand;

mod event;
mod queue;
mod schedule;

use queue::Queue;
use schedule::Schedule;
use event::*;

fn main() {
    let queue = Queue::new();
    let mut schedule = Schedule::<Queue>::new(queue);
    schedule.schedule_event(ScheduledEvent::new(Event::Arrival, 0.0));
    schedule.run();
}
