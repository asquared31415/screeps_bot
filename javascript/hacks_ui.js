// This is what is injected into the client. It may run multiple times, so the client should track
// whether it was actually run.
const code = `
(function () {
    window.client_hacks = window.client_hacks || {};
    if (window.client_hacks.injected) {
        return;
    }

    window.client_hacks.injected = true;

    window.DomHelper = window.DomHelper || {};
    (function (DomHelper) {
        DomHelper.generateCompiledElement = function (parent, content) {
            let $scope = parent.scope();
            let $compile = parent.injector().get("$compile");
            return $compile(content)($scope);
        }
    })(DomHelper);

    /* inject a tab into the bottom panel */
    let nav_tabs = angular.element('div.editor-panel ul.nav-tabs');

    /* note: the <tab> element is an angular directive declared by the game */
    const injectButton = \`<tab class="client-hacks-tab ng-scope" heading="Client Hacks" name="console" select="EditorPanel.hacksClick()"></tab>\`;

    nav_tabs.append(DomHelper.generateCompiledElement(nav_tabs, injectButton));

    /* add in code to select the tab when the button is clicked */
    /*let editor_panel = nav_tabs.scope().$parent.EditorPanel;
    if (!editor_panel.hacksClick) {
        editor_panel.hacksClick = function () {
            console.log("AAAAAAAAAAAAAAAAAAAAAA");
            alert("AAAAAAAAAAAAAA");
        }
    }
    */
})();
`;

// This runs in server side JS
global.inject_hacks_ui = function () {
    if (!global.ui_injected) {
        let replaced = code.replace(/(\r\n|\n|\r)\t+|(\r\n|\n|\r) +|(\r\n|\n|\r)/gm, '')
        console.log(`<span> injecting client hacks UI</span><script id="hacks_ui_script">${replaced};</script>`);
        global.ui_injected = true;
    }
}
global.force_inject_hacks_ui = function () {
    global.ui_injected = false;
    global.inject_hacks_ui();
}

global.inject_hacks_ui();
