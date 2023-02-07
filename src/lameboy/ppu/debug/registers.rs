use imgui::{Condition, Ui, Window};
use lameboy::ppu::registers::StatusInterruptFlags;
use lameboy::ppu::Ppu;

pub fn registers_window(ppu: &Ppu, ui: &Ui) {
    Window::new(im_str!("PPU-registers"))
        .size([224.0, 230.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            ui.text(im_str!("Control: {:?}", ppu.registers.control));
            ui.text(im_str!(
                "Status: {:?} - {:?}",
                ppu.combine_status_mode(),
                StatusInterruptFlags::from_bits_truncate(ppu.combine_status_mode())
            ));
            ui.text(im_str!("Scroll Y: {:?}", ppu.registers.scroll_y));
            ui.text(im_str!("Scroll X: {:?}", ppu.registers.scroll_x));
            ui.text(im_str!("LY: {:?}", ppu.registers.ly));
            ui.text(im_str!("LYC: {:?}", ppu.registers.lyc));
            ui.text(im_str!("DMA: {:?}", ppu.registers.dma));
            ui.text(im_str!("BG Palette: {:?}", ppu.registers.bg_palette));
            ui.text(im_str!("OBJ0 Palette: {:?}", ppu.registers.obj0_palette));
            ui.text(im_str!("OBJ1 Palette: {:?}", ppu.registers.obj1_palette));
            ui.text(im_str!("Window Y: {:?}", ppu.registers.window_y));
            ui.text(im_str!("Window X: {:?}", ppu.registers.window_x));
        });
}
