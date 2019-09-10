use gui::imgui_debug_state::ImguiDebugState;
use imgui::Ui;

pub trait ImguiDebuggable {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebugState);
}
