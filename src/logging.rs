use core::panic::PanicInfo;
use js_sys::JsString;
use log::{error, Log};
use std::panic;
use web_sys::console as js_console;

const TRACE_COLOR: &str = "#999999";
const DEBUG_COLOR: &str = "#008c96";
const INFO_COLOR: &str = "#dddddd";
const WARN_COLOR: &str = "#f2d99a";
const WARN_BG_COLOR: &str = "#42381f";
const ERR_COLOR: &str = "#cf90a8";
const ERR_BG_COLOR: &str = "#4b2f36";

struct JsLogger;

impl Log for JsLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                log::Level::Trace => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{TRACE_COLOR}\">[TRACE] {}</span>",
                    record.args()
                ))),
                log::Level::Debug => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{DEBUG_COLOR}\">[DEBUG] {}</style>",
                    record.args()
                ))),
                log::Level::Info => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{INFO_COLOR}\">[INFO] {}",
                    record.args()
                ))),
                log::Level::Warn => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{WARN_COLOR};background-color:{WARN_BG_COLOR}\">⚠️ [WARN] {}</span>",
                    record.args()
                ))),
                log::Level::Error => js_console::log_1(&JsString::from(format!(
                    "<span style=\"color:{ERR_COLOR};background-color:{ERR_BG_COLOR}\">⛔ [ERROR] {}</span>",
                    record.args()
                ))),
            }
        }
    }

    fn flush(&self) {}
}

pub fn setup_logger() {
    log::set_logger(&JsLogger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    panic::set_hook(Box::new(panic_hook));
}

fn panic_hook(info: &PanicInfo) {
    error!("{info}");
}
