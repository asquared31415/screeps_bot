function inject() {
    if (!global.clientVisualizerInjected) {
        $("section.game").append("<div style=\"position: absolute; left: 80px; margin:10px; z-index: 100;\">this is an element injected into the client!</div>");
        global.clientVisualizerInjected = true;
    }
}

global.forceInjectClientVisualizer = function () {
    global.clientVisualizerInjected = false;
    inject();
};

inject();