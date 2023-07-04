/* Note: this file is intended to be run constantly to be sure that the UI is always available */

/* TODO: put this in bottom bar as a tab */
let left_controls = $("section.room div.left-controls");
let existing_ui = left_controls.children("#client_hacks_ui")[0];
if (!existing_ui) {
    left_controls.append("<div id=\"client_hacks_ui\">new UI here</div>");
}