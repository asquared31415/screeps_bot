use std::collections::VecDeque;

/// The maximum number of ticks to store data for.
/// There will actually be `min(data_length, (now - start_time))` entries stored at any time.
const STORAGE_TIME: usize = 32_usize;

pub struct GlobalStats {
    /// The Game.time in which the bot was initialized.
    global_start: u32,

    /// The stored data for each tick.
    data: VecDeque<TickStats>,

    /// An optimization to calculate the sum of the used CPU without needing to loop over it all.
    sum_cpu_usage: f64,
}

impl GlobalStats {
    pub fn new(global_start: u32) -> Self {
        Self {
            global_start,
            data: VecDeque::with_capacity(STORAGE_TIME),
            sum_cpu_usage: 0_f64,
        }
    }

    pub fn push_tick_data(&mut self, stats: TickStats) {
        // Remove the oldest entry if adding the current entry would be too much.
        if self.data.len() >= STORAGE_TIME {
            if let Some(oldest) = self.data.pop_front() {
                self.sum_cpu_usage -= oldest.cpu();
            }
        }

        self.sum_cpu_usage += stats.cpu();
        self.data.push_back(stats);
    }

    pub fn num_ticks(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn global_start(&self) -> u32 {
        self.global_start
    }

    pub fn total_cpu(&self) -> f64 {
        self.sum_cpu_usage
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
    /// Time in milliseconds since unix epoch.
    real_time: f64,
    cpu_usage: f64,
}

impl TickStats {
    pub fn new(tick: u32, real_time: f64, cpu_usage: f64) -> Self {
        Self {
            tick,
            real_time,
            cpu_usage,
        }
    }

    pub fn tick(&self) -> u32 {
        self.tick
    }

    pub fn real_time(&self) -> f64 {
        self.real_time
    }

    pub fn cpu(&self) -> f64 {
        self.cpu_usage
    }
}
