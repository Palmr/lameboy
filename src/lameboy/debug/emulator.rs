use imgui::{Condition, Ui, Window};
use lameboy::Lameboy;

pub fn emulator_window<'a>(lameboy: &mut Lameboy, ui: &Ui<'a>) {
    Window::new(im_str!("Emulator"))
        .size([255.0, 75.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            if ui.button(im_str!("Reset"), [0.0, 0.0]) {
                lameboy.reset();
            }
            ui.same_line(0.0);
            if ui.button(im_str!("Step"), [0.0, 0.0]) {
                lameboy.step();
            }
            ui.same_line(0.0);
            if ui.button(im_str!("Continue"), [0.0, 0.0]) {
                lameboy.step();
                lameboy.running = true;
            }
            ui.same_line(0.0);
            ui.checkbox(im_str!("running"), &mut lameboy.running);

            if ui.button(im_str!("Dump PC history"), [0.0, 0.0]) {
                info!("Dumping PC history");
                for i in 0..lameboy.get_cpu().pc_history.len() {
                    let hp = lameboy.get_cpu().pc_history_pointer.wrapping_add(i)
                        % lameboy.get_cpu().pc_history.len();
                    info!("[{}] - PC = 0x{:04X}", i, lameboy.get_cpu().pc_history[hp]);
                }
            }

            ui.input_int(im_str!("Trace Count"), &mut lameboy.trace_count)
                .chars_decimal(true)
                .build();
        });
}
