use crate::lameboy::ppu::registers::StatusInterruptFlags;
use crate::lameboy::ppu::Ppu;
use imgui::{Condition, Ui};

pub fn registers_window(ppu: &Ppu, ui: &Ui) {
    ui.window("PPU-registers")
        .size([224.0, 230.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            ui.text(format!("Control: {:?}", ppu.registers.control));
            ui.text(format!(
                "Status: {:?} - {:?}",
                ppu.combine_status_mode(),
                StatusInterruptFlags::from_bits_truncate(ppu.combine_status_mode())
            ));
            ui.text(format!("Scroll Y: {:?}", ppu.registers.scroll_y));
            ui.text(format!("Scroll X: {:?}", ppu.registers.scroll_x));
            ui.text(format!("LY: {:?}", ppu.registers.ly));
            ui.text(format!("LYC: {:?}", ppu.registers.lyc));
            ui.text(format!("DMA: {:?}", ppu.registers.dma));
            ui.text(format!("BG Palette: {:?}", ppu.registers.bg_palette));
            ui.text(format!("OBJ0 Palette: {:?}", ppu.registers.obj0_palette));
            ui.text(format!("OBJ1 Palette: {:?}", ppu.registers.obj1_palette));
            ui.text(format!("Window Y: {:?}", ppu.registers.window_y));
            ui.text(format!("Window X: {:?}", ppu.registers.window_x));
        });
}
