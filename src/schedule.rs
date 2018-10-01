use event::*;
use system::SimulatedSystem;

pub struct Schedule<T> {
    event_list : Vec<ScheduledEvent>,
    time_now: f64,
    sim_system: T,
}

impl<T: SimulatedSystem> Schedule<T> {
    pub fn new<S: SimulatedSystem>(sim_system: S) -> Schedule<S> {
        Schedule {
            event_list: Vec::new(),
            time_now: 0.0,
            sim_system,
        }
    }

    pub fn schedule_event(&mut self, event: ScheduledEvent) {
        let mut index = 0;

        loop {
            if index >= self.event_list.len() {
                self.event_list.push(event);
                break;
            }

            if self.event_list[index].time > event.time {
                self.event_list.insert(index,event);
                break;
            }
            index += 1;
        }
    }

    pub fn run(&mut self) {
        while self.event_list.len() > 0 {
            let next_event = self.event_list.remove(0);
            self.time_now = next_event.time;

            let new_events =
                self.sim_system.handle_event(&next_event.event, self.time_now);

            for event in new_events {
                self.schedule_event(event);
            }

            match next_event.event {
                Event::Stop => break,
                _ => continue,
            }
        }

        println!("Simulation complete at time {:.2}", self.time_now);
        self.sim_system.print_results();
    }
}