mod statistics;
mod time_gen;

use event::*;
use system::SimulatedSystem;
use self::statistics::*;
use self::time_gen::*;

pub struct Queue {
    length: u32,
    busy: bool,
    arrival_times: Generator,
    service_times: Generator,
    stats: Stats,
}

impl Queue {
    #[allow(dead_code)]
    pub fn new(arrival_rate: f64, service_rate: f64) -> Queue {
        Queue {
            length: 0,
            busy: false,
            arrival_times: Generator::new(arrival_rate),
            service_times: Generator::new(service_rate),
            stats: Stats::new(),
        }
    }

    fn handle_arrival(&mut self, time_now: f64) -> Vec<ScheduledEvent> {
        let mut next_events: Vec<ScheduledEvent> = Vec::new();

        if !self.busy {
            self.busy = true;
            self.generate_next_departure(&mut next_events, time_now);
        } else {
            self.length += 1;
        }
        self.generate_next_arrival(&mut next_events, time_now);

        next_events
    }

    fn handle_departure(&mut self, time_now: f64) -> Vec<ScheduledEvent> {
        let mut next_events: Vec<ScheduledEvent> = Vec::new();

        if self.length > 0 {
            self.length -= 1;
            self.generate_next_departure(&mut next_events, time_now);
        } else {
            self.busy = false;
        }

        next_events
    }

    fn generate_next_arrival(&mut self, event_list: &mut Vec<ScheduledEvent>, time_now: f64) {
        event_list.push(
            ScheduledEvent::new(Event::Arrival, time_now + self.arrival_times.get_next()));
    }

    fn generate_next_departure(&mut self, event_list: &mut Vec<ScheduledEvent>, time_now: f64) {
        event_list.push(
            ScheduledEvent::new(Event::Departure, time_now + self.service_times.get_next()));
    }
}

impl SimulatedSystem for Queue {
    fn handle_event(&mut self, event: &Event, time_now: f64) -> Vec<ScheduledEvent> {
        let event = *event;
        let next_events = match event {
            Event::Arrival => self.handle_arrival(time_now),
            Event::Departure => self.handle_departure(time_now),
            _ => vec![],
        };

        self.stats.store(DataPoint {
            event,
            queue_len: self.length,
            busy: self.busy,
            time: time_now,
        });

        next_events
    }

    fn print_results(&self) {
        println!("*******************************************************");
        println!("Simulation results:");
        println!("   Average length of the queue = {:.2}",
                 self.stats.get_avg_length());
        println!("   Percent time with 3 or more in the queue = {:.2}%",
                 self.stats.get_perc_time_queue_length(3));
        println!("   Total server utilization = {:.2}",
                 self.stats.get_server_utilization());
    }
}