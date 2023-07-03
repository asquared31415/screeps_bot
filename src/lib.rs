#![feature(int_roundings, local_key_cell_methods)]

use crate::stats::{GlobalStats, TickStats};
use crate::visualization::UiVisualizer;
use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};
use js_sys::Date;
use log::*;
use screeps::game;
use wasm_bindgen::prelude::*;

mod logging;
mod stats;
mod visualization;

static INIT_TICK: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub fn init() {
    logging::setup_logger();
    info!("Initializing...");

    // store the init tick so that it can be skipped in stats
    let tick = game::time();
    INIT_TICK.store(tick, Ordering::Relaxed);

    // memory::MEM.with_borrow(|mem| {
    //     tasks::init(mem);
    // });
}

// keep the last DATA_TIME ticks in global stats
const DATA_TIME: u32 = 128;
thread_local! {
    static STATS: RefCell<GlobalStats> = RefCell::new(GlobalStats::new(DATA_TIME));
}

#[wasm_bindgen]
pub fn game_loop() {
    let tick = game::time();
    let bucket = game::cpu::bucket();
    info!("Starting game tick {} with {} bucket", tick, bucket);

    if bucket >= 10_000 {
        // generate a pixel and skip this tick
        match game::cpu::generate_pixel() {
            Ok(()) => {
                return;
            }
            Err(_) => {
                warn!("We had at least 10_000 bucket, but the game claimed there wasn't enough");
            }
        }
    }

    let cpu_usage_before_stats = game::cpu::get_used();

    // Handle stats
    STATS.with_borrow_mut(|stats| {
        if tick == INIT_TICK.load(Ordering::Relaxed) {
            info!("Not updating stats for initial tick {}", tick);
        } else {
            let tick_stats = TickStats::new(tick, Date::new_0(), cpu_usage_before_stats);

            stats.push_tick_data(tick_stats);
        }

        for room in game::rooms().values() {
            let room = room.name();
            debug!("Drawing UI stats in room {}", room);
            let mut visualizer = UiVisualizer::new(room);
            visualizer.draw_stats(stats);
        }
    });

    // get CPU again to count the time spent drawing stats
    info!("Ending tick {}: {:.3} CPU", tick, game::cpu::get_used());
}
