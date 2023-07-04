use js_sys::JsString;
use log::*;
use regex::Regex;
use std::sync::LazyLock;

pub fn inject_all() {
    VISUALIZER_HACK.inject();

    // This should be last so that it can find everything
    UI_HACK.inject();
}

// Creates a UI for controling client hacks
static UI_HACK: LazyLock<ClientHack> =
    LazyLock::new(|| ClientHack::new("hacks_ui", include_str!("../javascript/client/hacks_ui.js")));

static VISUALIZER_HACK: LazyLock<ClientHack> = LazyLock::new(|| {
    ClientHack::new(
        "visualizer_hack",
        include_str!("../javascript/client/visualizer.js"),
    )
});

#[derive(Debug)]
/// A utility to inject code into the client.
/// The provided JS code should be resilient to being run multiple times.
struct ClientHack {
    name: &'static str,
    inject_code: String,
}

impl ClientHack {
    pub fn new(name: &'static str, code: &str) -> Self {
        static NEWLINE_REPLACER: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\n\t+|\n +|\n").unwrap());

        let replaced = NEWLINE_REPLACER.replace_all(code, "");
        if replaced.contains("</script>") {
            warn!("Client hack {} contains a closing script tag!", name);
            // TODO: set err status
        }

        // this removes the script element just to be sure it cleaned up
        format!(
            "<script>{};document.currentScript.remove();</script>",
            replaced
        );
        Self {
            name,
            inject_code: replaced.into_owned(),
        }
    }

    pub fn inject(&self) {
        debug!("Injecting client hack {}", self.name);

        // using raw log here because that makes sure that it always gets
        // sent, unlike the log macros
        web_sys::console::log_1(&JsString::from(self.inject_code.as_str()));
        debug!("injected {}", self.name);
    }
}
