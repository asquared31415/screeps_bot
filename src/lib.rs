#![feature(int_roundings, lazy_cell, const_option)]

use crate::stats::{GlobalStats, TickStats};
use crate::visualization::UiVisualizer;
use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};
use js_sys::Date;
use log::*;
use screeps::{game, PIXEL_CPU_COST};
use wasm_bindgen::prelude::*;

mod inventory;
mod logging;
mod stats;
mod task;
mod visualization;

static INIT_TICK: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub fn init() {
    logging::setup_logger();
    info!("Initializing...");

    // store the init tick so that it can be skipped in stats
    let tick = game::time();
    INIT_TICK.store(tick, Ordering::Relaxed);

    // STATS should never be borrowed on init, so this should not panic.
    STATS.set(Some(GlobalStats::new(tick)));

    // memory::MEM.with_borrow(|mem| {
    //     tasks::init(mem);
    // });
}

thread_local! {
    static STATS: RefCell<Option<GlobalStats>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn game_loop() {
    let tick = game::time();
    let bucket = game::cpu::bucket();
    info!("Starting game tick {} with {} bucket", tick, bucket);

    if try_generate_pixel() {
        info!("Generated a pixel! Skipping this tick");
        process_stats(tick);
        return;
    }

    process_stats(tick);
    // get CPU again to count the time spent drawing stats
    info!("Ending tick {}: {:.3} CPU", tick, game::cpu::get_used());
}

fn try_generate_pixel() -> bool {
    #[cfg(feature = "pixels")]
    fn generate() -> bool {
        // generate a pixel and skip this tick
        match game::cpu::generate_pixel() {
            Ok(()) => true,
            Err(_) => {
                warn!("We had at least 10_000 bucket, but the game claimed there wasn't enough");
                false
            }
        }
    }

    #[cfg(not(feature = "pixels"))]
    fn generate() -> bool {
        trace!("could generate a pixel but pixels not enabled");
        false
    }

    let bucket = game::cpu::bucket();
    // Don't try to run the pixel
    if bucket >= PIXEL_CPU_COST as i32 {
        generate()
    } else {
        false
    }
}

fn process_stats(tick: u32) {
    if tick == INIT_TICK.load(Ordering::Relaxed) {
        info!("Not updating stats for initial tick {}", tick);
        return;
    }

    let cpu_usage_before_stats = game::cpu::get_used();
    STATS.with_borrow_mut(|stats| {
        // Stats should always be initialized by this point.
        let stats = stats.as_mut().unwrap();

        let tick_stats = TickStats::new(tick, Date::new_0().value_of(), cpu_usage_before_stats);

        stats.push_tick_data(tick_stats);

        debug!("Drawing UI stats");
        let mut visualizer = UiVisualizer::new(None);
        visualizer.draw_stats(&stats);
    });
}
