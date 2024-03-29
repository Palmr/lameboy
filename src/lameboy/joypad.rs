use imgui::{Condition, Ui};

use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::gui::imgui_debuggable::ImguiDebuggable;
use crate::lameboy::mmu::mmuobject::MmuObject;

const LOW_NIBBLE_MASK: u8 = 0x0F;
const COLUMN_MASK: u8 = 0b0011_0000;
const COLUMN_BUTTON_KEYS: u8 = 0b0001_0000;
const COLUMN_DIRECTION_KEYS: u8 = 0b0010_0000;

pub struct Joypad {
    selected_column: u8,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            selected_column: 0,
            a: false,
            b: false,
            start: false,
            select: false,
            right: false,
            left: false,
            up: false,
            down: false,
        }
    }

    fn direction_byte(&self) -> u8 {
        let mut joyp = LOW_NIBBLE_MASK;

        if self.down {
            joyp &= 0b0111
        }
        if self.up {
            joyp &= 0b1011
        }
        if self.left {
            joyp &= 0b1101
        }
        if self.right {
            joyp &= 0b1110
        }

        joyp
    }

    fn button_byte(&self) -> u8 {
        let mut joyp = LOW_NIBBLE_MASK;

        if self.start {
            joyp &= 0b0111
        }
        if self.select {
            joyp &= 0b1011
        }
        if self.b {
            joyp &= 0b1101
        }
        if self.a {
            joyp &= 0b1110
        }

        joyp
    }
}

impl MmuObject for Joypad {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => match self.selected_column {
                COLUMN_BUTTON_KEYS => 0xC0 | self.selected_column | self.button_byte(),
                COLUMN_DIRECTION_KEYS => 0xC0 | self.selected_column | self.direction_byte(),
                _ => 0xC0 | self.selected_column | LOW_NIBBLE_MASK,
            },
            _ => panic!("Attempted to access [RD] Joypad from an invalid address: {addr:#X}"),
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0xFF00 => {
                self.selected_column = data & COLUMN_MASK;
            }
            _ => panic!("Attempted to access [WR] Joypad from an invalid address: {addr:#X}"),
        }
    }
}

impl ImguiDebuggable for Joypad {
    fn imgui_display(&mut self, ui: &Ui, _imgui_debug: &mut ImguiDebugState) {
        ui.window("Joypad")
            .size([150.0, 115.0], Condition::FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.text(format!("A = {}", self.a));
                ui.text(format!("B = {}", self.b));
                ui.text(format!("Select = {}", self.select));
                ui.text(format!("Start = {}", self.start));
                ui.text(format!("JOYP = 0B{:04b}", self.button_byte()));

                ui.separator();

                ui.text(format!("Right = {}", self.right));
                ui.text(format!("Left = {}", self.left));
                ui.text(format!("Up = {}", self.up));
                ui.text(format!("Down = {}", self.down));
                ui.text(format!("JOYP = 0B{:04b}", self.direction_byte()));
            });
    }
}
