use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::{PKG_AUTHORS, PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};
use imgui::{Condition, Ui};

pub fn about_window(ui: &Ui, imgui_debug: &mut ImguiDebugState) {
    ui.window(&format!("About - {PKG_NAME} v{PKG_VERSION}"))
        .size([250.0, 100.0], Condition::Always)
        .collapsible(false)
        .resizable(false)
        .build(|| {
            ui.text(PKG_DESCRIPTION);
            ui.text(PKG_AUTHORS);
            if ui.button_with_size("Close", [75.0, 30.0]) {
                imgui_debug.show_about = false;
            }
        });
}
