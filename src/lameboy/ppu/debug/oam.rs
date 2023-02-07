use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::lameboy::ppu::sprite::Sprite;
use crate::lameboy::ppu::Ppu;
use imgui::{Condition, Ui};

pub fn oam_window(ppu: &Ppu, ui: &Ui, imgui_debug: &mut ImguiDebugState) {
    ui.window("PPU-OAM")
        .size([224.0, 230.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            ui.input_int("Sprite Index", &mut imgui_debug.ppu_sprite_index)
                .build();
            // Limit index
            if imgui_debug.ppu_sprite_index < 0 {
                imgui_debug.ppu_sprite_index = 0
            };
            if imgui_debug.ppu_sprite_index > 39 {
                imgui_debug.ppu_sprite_index = 39
            };

            let sprite = Sprite::new(ppu, imgui_debug.ppu_sprite_index as u8);
            ui.text(format!("Position: {:?}, {:?}", sprite.x, sprite.y));
            ui.text(format!("Tile: {:?}", sprite.tile_index));
            ui.text(format!("Flip X: {:?}", sprite.flip_x));
            ui.text(format!("Flip Y: {:?}", sprite.flip_y));
            ui.text(format!("Priority: {:?}", sprite.priority));
        });
}
