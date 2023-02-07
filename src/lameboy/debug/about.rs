use gui::imgui_debug_state::ImguiDebugState;
use imgui::{Condition, Ui, Window};

use {PKG_AUTHORS, PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub fn about_window(ui: &Ui, imgui_debug: &mut ImguiDebugState) {
    Window::new(&im_str!("About - {} v{}", PKG_NAME, PKG_VERSION))
        .size([250.0, 100.0], Condition::Always)
        .collapsible(false)
        .resizable(false)
        .build(ui, || {
            ui.text(im_str!("{}", PKG_DESCRIPTION));
            ui.text(im_str!("{}", PKG_AUTHORS));
            if ui.button(im_str!("Close"), [75.0, 30.0]) {
                imgui_debug.show_about = false;
            }
        });
}
