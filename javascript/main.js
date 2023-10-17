const MODULE_NAME = "screeps_bot_v3";

// Set to true to have JS call Game.cpu.halt() on the next tick it processes.
// This is used so that console output from the end of the erroring tick
// will still be emitted, since calling halt destroys the environment instantly.
// The environment will be re-created with a fresh heap next tick automatically.
// We lose a tick of processing here, but it should be exceptional that code
// throws at all.
let halt_next_tick = false;

let wasm_module;
module.exports.loop = function () {
    try {
        if (halt_next_tick) {
            // We encountered an error, skip execution in this tick and get
            // a new environment next tick.
            Game.cpu.halt();
            return;
        }

        if (wasm_module) {
            wasm_module.game_loop();
        } else {
            console.log("[JS] Module not loaded... loading");

            const BUCKET_TO_COMPILE = 500;

            let bucket = Game.cpu.bucket;
            if (bucket < BUCKET_TO_COMPILE) {
                console.log(`[JS] ${bucket}/${BUCKET_TO_COMPILE} bucket to compile wasm`);
            } else {
                let cpu_before = Game.cpu.getUsed();
                // delete the module from the cache, so we can reload it
                if (MODULE_NAME in require.cache) {
                    delete require.cache[MODULE_NAME];
                }

                console.log("[JS] Compiling...");
                // load the wasm module
                wasm_module = require(MODULE_NAME);
                // load the wasm instance!
                wasm_module.initialize_instance();

                let cpu_after = Game.cpu.getUsed();
                console.log(`[JS] ${cpu_after - cpu_before}cpu used to initialize wasm`);

                // run the setup function, which configures logging
                wasm_module.init();
                // go ahead and run the loop for its first tick
                wasm_module.game_loop();
            }
        }
    } catch (e) {
        if (e instanceof WebAssembly.CompileError || e instanceof WebAssembly.LinkError) {
            console.log(`[JS] exception during wasm creation: ${e}`);
        } else if (e instanceof WebAssembly.RuntimeError) {
            console.log(`[JS] wasm aborted`);
        } else {
            console.log(`[JS] unexpected exception: ${e.stack}`);
        }
        console.log(`[JS] destroying environment...`);

        // reset everything
        halt_next_tick = true;
    }
}