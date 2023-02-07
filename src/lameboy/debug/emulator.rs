use crate::lameboy::Lameboy;
use imgui::{Condition, Ui};

pub fn emulator_window(lameboy: &mut Lameboy, ui: &Ui) {
    ui.window("Emulator")
        .size([255.0, 75.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            if ui.button("Reset") {
                lameboy.reset();
            }
            ui.same_line();
            if ui.button("Step") {
                lameboy.step();
            }
            ui.same_line();
            if ui.button("Continue") {
                lameboy.step();
                lameboy.running = true;
            }
            ui.same_line();
            ui.checkbox("running", &mut lameboy.running);

            if ui.button("Dump PC history") {
                info!("Dumping PC history");
                for i in 0..lameboy.get_cpu().pc_history.len() {
                    let hp = lameboy.get_cpu().pc_history_pointer.wrapping_add(i)
                        % lameboy.get_cpu().pc_history.len();
                    info!("[{}] - PC = 0x{:04X}", i, lameboy.get_cpu().pc_history[hp]);
                }
            }

            ui.input_int("Trace Count", &mut lameboy.trace_count)
                .chars_decimal(true)
                .build();
        });
}
