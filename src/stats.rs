use js_sys::Date;
use std::collections::VecDeque;

pub struct GlobalStats {
    /// The maximum number of ticks that this should store data for.
    /// There will actually be `min(data_length, (now - start_time))` entries stored at any time.
    data_length: u32,

    data: VecDeque<TickStats>,
}

impl GlobalStats {
    pub fn new(data_length: u32) -> Self {
        Self {
            data_length,
            data: VecDeque::with_capacity(usize::try_from(data_length).unwrap()),
        }
    }

    pub fn push_tick_data(&mut self, stats: TickStats) {
        // Remove the oldest entry if adding the current entry would be too much.
        if self.data.len() as u32 >= self.data_length {
            self.data.pop_front();
        }

        self.data.push_back(stats);
    }

    pub fn num_ticks(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn iter(&self) -> impl Iterator<Item = &TickStats> {
        self.data.iter()
    }

    pub fn get(&self, idx: u32) -> Option<&TickStats> {
        self.data.get(idx as usize)
    }
}

#[derive(Debug)]
pub struct TickStats {
    tick: u32,
    real_time: Date,
    cpu_usage: f64,
}

impl TickStats {
    pub fn new(tick: u32, real_time: Date, cpu_usage: f64) -> Self {
        Self {
            tick,
            real_time,
            cpu_usage,
        }
    }

    pub fn tick(&self) -> u32 {
        self.tick
    }

    pub fn real_time(&self) -> &Date {
        &self.real_time
    }

    pub fn cpu(&self) -> f64 {
        self.cpu_usage
    }
}
