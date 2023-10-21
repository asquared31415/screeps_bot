use crate::stats::GlobalStats;
use log::{debug, info, trace};
use screeps::{game, RoomName, RoomVisual, TextAlign, TextStyle, CPU_BUCKET_MAX};

pub struct UiVisualizer {
    room: Option<RoomName>,
    visual: RoomVisual,
    line: u32,
}

impl UiVisualizer {
    /// Create a visualizer for the specified room, or use all rooms if not specified
    pub fn new(room: Option<RoomName>) -> Self {
        Self {
            room,
            visual: RoomVisual::new(room),
            line: 0,
        }
    }

    pub fn draw_stats(&mut self, stats: &GlobalStats) {
        // Track how long was spent showing the stats
        let start_cpu = game::cpu::get_used();

        let style = Some(
            TextStyle::default()
                .align(TextAlign::Left)
                .custom_font("0.7 monospace"),
        );

        let num_ticks = stats.num_ticks();
        debug!("Global stats have {} ticks of data", num_ticks);
        if num_ticks == 0 {
            self.draw_line(format!("No Data"), style.clone());
            return;
        }

        // check above ensures that there's at least one tick
        let start_millis = stats.get(0).unwrap().real_time();
        let end_millis = stats.get(num_ticks - 1).unwrap().real_time();
        let avg_tick_time = (end_millis - start_millis) / f64::from(num_ticks) / 1000.0;

        self.draw_line(format!("tick: {}", game::time()), style.clone());
        self.draw_line(
            format!("global reset at {}", stats.global_start()),
            style.clone(),
        );

        self.draw_line(format!("{} ticks of data", num_ticks), style.clone());
        self.draw_line(format!("time: {:.3}s", avg_tick_time), style.clone());

        let cpu_usage = stats.total_cpu() / f64::from(num_ticks);
        let cpu_limit = game::cpu::limit();
        self.draw_cpu(cpu_usage, cpu_limit, style.clone());

        let heap_stats = game::cpu::get_heap_statistics();
        let used_heap = heap_stats.total_heap_size();
        let max_heap = heap_stats.heap_size_limit();
        trace!("Heap: {:#8X}/{:#8X}", used_heap, max_heap);

        self.draw_line(
            format!("heap: {:5.2}%", (used_heap as f64 / max_heap as f64)),
            style.clone(),
        );

        let end_cpu = game::cpu::get_used();
        info!("Used {:>1.3}cpu for visualizer UI", end_cpu - start_cpu,);
    }

    fn draw_cpu(&mut self, cpu_usage: f64, cpu_limit: u32, style: Option<TextStyle>) {
        let cpu_percent = cpu_usage / (cpu_limit as f64);
        let color_str = if cpu_percent > 1.0 {
            "#FF0000"
        } else if cpu_percent > 0.75 {
            "#FF8000"
        } else if cpu_percent > 0.5 {
            "#FFFF00"
        } else {
            "#80FF00"
        };
        let cpu_text_style = style.clone().map(|s| s.color(color_str));

        // the longest possible CPU value is 499.999, so reserve space for that
        self.draw_line(String::from("cpu: "), style.clone());
        self.same_line(
            format!("    {:>7.3}/{}", cpu_usage, cpu_limit),
            cpu_text_style.clone(),
        );
        self.draw_line(
            format!("bucket:{:>5}/{}", game::cpu::bucket(), CPU_BUCKET_MAX),
            style.clone(),
        );
    }

    fn draw_line(&mut self, text: String, style: Option<TextStyle>) {
        let y_offset = self.line as f32 + 0.15_f32;
        self.visual.text(0.0, y_offset, text, style);
        self.line += 1;
    }

    fn same_line(&mut self, text: String, style: Option<TextStyle>) {
        self.line -= 1;
        self.draw_line(text, style);
    }
}
