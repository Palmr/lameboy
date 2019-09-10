use gui::imgui_debug_state::ImguiDebugState;
use imgui::{Condition, Ui, Window};
use ppu::sprite::Sprite;
use ppu::PPU;

pub fn oam_window<'a>(ppu: &PPU, ui: &Ui<'a>, imgui_debug: &mut ImguiDebugState) {
    Window::new(im_str!("PPU-OAM"))
        .size([224.0, 230.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            ui.input_int(im_str!("Sprite Index"), &mut imgui_debug.ppu_sprite_index)
                .build();
            // Limit index
            if imgui_debug.ppu_sprite_index < 0 {
                imgui_debug.ppu_sprite_index = 0
            };
            if imgui_debug.ppu_sprite_index > 39 {
                imgui_debug.ppu_sprite_index = 39
            };

            let sprite = Sprite::new(ppu, imgui_debug.ppu_sprite_index as u8);
            ui.text(im_str!("Position: {:?}, {:?}", sprite.x, sprite.y));
            ui.text(im_str!("Tile: {:?}", sprite.tile_index));
            ui.text(im_str!("Flip X: {:?}", sprite.flip_x));
            ui.text(im_str!("Flip Y: {:?}", sprite.flip_y));
            ui.text(im_str!("Priority: {:?}", sprite.priority));
        });
}
