"use strict";

const MODULE_NAME = "screeps_bot_v3";

// This provides the function `console.error` that wasm_bindgen sometimes expects to exist,
// especially with type checks in debug mode. An alternative is to have this be `function () {}`
// and let the exception handler log the thrown JS exceptions, but there is some additional
// information that wasm_bindgen only passes here.
//
// There is nothing special about this function and it may also be used by any JS/Rust code as a convenience.
function console_error() {
    const processedArgs = _.map(arguments, (arg) => {
        if (arg instanceof Error) {
            // On this version of Node, the `stack` property of errors contains
            // the message as well.
            return arg.stack;
        } else {
            return arg;
        }
    }).join(' ');
    console.log("ERROR:", processedArgs);
    Game.notify(processedArgs);
}


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

        // need to freshly override the fake console object each tick
        console.error = console_error;

        // Don't let the game interfere with our memory or incur parsing, but let it work for moveTo and friends.
        // This causes the game's view of Memory to effectively be tied to global instead, much like memhack.
        delete global.Memory;
        global.TempMemory = global.TempMemory || Object.create(null);
        global.Memory = global.TempMemory;

        if (wasm_module) {
            wasm_module.game_loop();
        } else {
            console.log("[JS] Module not loaded... loading");

            // Only load the wasm module if there is enough bucket to complete it this tick.
            const BUCKET_TO_COMPILE = 500;
            let bucket = Game.cpu.bucket;
            if (bucket < BUCKET_TO_COMPILE) {
                console.log(`[JS] ${bucket}/${BUCKET_TO_COMPILE} bucket to compile wasm`);
                return;
            }

            let cpu_before = Game.cpu.getUsed();

            console.log("[JS] Compiling...");
            // load the wasm module
            wasm_module = require(MODULE_NAME);
            // load the wasm instance!
            wasm_module.initialize_instance();

            let cpu_after = Game.cpu.getUsed();
            console.log(`[JS] ${cpu_after - cpu_before}cpu used to initialize wasm`);

            // run the setup function, which configures logging
            wasm_module.init();
            // TODO: consider not running this if there's not enough bucket.
            // run the loop for its first tick
            wasm_module.game_loop();

        }
    } catch (e) {
        if (e instanceof WebAssembly.CompileError || e instanceof WebAssembly.LinkError) {
            console.log(`[JS] exception during wasm compilation: ${e}`);
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