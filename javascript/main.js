const MODULE_NAME = "screeps_bot_v3";

let wasm_module;
module.exports.loop = function () {
    try {
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
        console.log(e);
        if (e.stack) {
            console.log(e.stack);
        }
        // reset everything
        wasm_module = null;
    }
}