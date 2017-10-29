use event::Event;


pub struct Stats {
    data_points: Vec<DataPoint>,
    total_sim_time: f64,
    total_busy_time: f64,
    total_queue_time: f64,
}

pub struct DataPoint {
    pub event: Event,
    pub queue_len: u32,
    pub busy: bool,
    pub time: f64,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            data_points: Vec::new(),
            total_sim_time: 0.0,
            total_busy_time: 0.0,
            total_queue_time: 0.0,
        }
    }

    pub fn store(&mut self, data_point: DataPoint) {
        self.total_sim_time = data_point.time;

        if let Some(prev_data_point) = self.data_points.last() {
            if prev_data_point.busy {
                self.total_busy_time += data_point.time - prev_data_point.time;
            }

            self.total_queue_time +=
                prev_data_point.queue_len as f64 * (self.total_sim_time - prev_data_point.time)
        }

        self.data_points.push(data_point);
    }

    pub fn get_avg_length(&self) -> f64 {
        safe_division(self.total_queue_time, self.total_sim_time)
    }

    pub fn get_perc_time_queue_length(&self, n: u32) -> f64 {
        if self.total_sim_time <= 0.0 || self.data_points.len() == 0 {
            0.0
        } else {
            let mut queue_time = 0.0;
            let mut data_iter = self.data_points.iter();
            let mut curr_point = data_iter.next().unwrap();

            loop {
                match data_iter.next() {
                    Some(next_point) => {
                        if curr_point.queue_len >= n {
                            queue_time += next_point.time - curr_point.time;
                        }
                        curr_point = next_point;
                    },
                    None => break,
                }
            }

            (queue_time / self.total_sim_time) * 100.0
        }
    }

    pub fn get_server_utilization(&self) -> f64 {
        safe_division(self.total_busy_time, self.total_sim_time)
    }
}

fn safe_division(n: f64, d: f64) -> f64 {
    if d <= 0.0 {
        0.0
    } else {
        n / d
    }
}