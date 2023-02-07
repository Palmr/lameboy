use crate::gui::imgui_debug_state::ImguiDebugState;
use imgui::Ui;

pub trait ImguiDebuggable {
    fn imgui_display(&mut self, ui: &Ui, imgui_debug: &mut ImguiDebugState);
}
