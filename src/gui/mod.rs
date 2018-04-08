use glium::{Display, Surface};
use glium::glutin;
use glium::glutin::EventsLoop;
use imgui::{ImGui, Ui, ImGuiKey, ImString};
use imgui_glium_renderer::Renderer;
use std::time::Instant;

use lameboy::Lameboy;

pub mod imguidebug;

use gui::imguidebug::ImguiDebug;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct GUI {
    pub display: Display,
    events_loop: EventsLoop,
    imgui: ImGui,
    renderer: Renderer,
    last_frame: Instant,
    mouse_state: MouseState,
    show_imgui: bool,
}

impl GUI {
    pub fn init(window_size: (u32, u32), rom_file_name: &str) -> GUI {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let window = glutin::WindowBuilder::new()
            .with_title(format!("{} - Lameboy - v0.1", rom_file_name))
            .with_dimensions(window_size.0, window_size.1);
        let display = Display::new(window, context, &events_loop).unwrap();

        let mut imgui = ImGui::init();
        imgui.set_ini_filename(Some(ImString::new("lameboy.ini")));
        let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

        imgui.set_imgui_key(ImGuiKey::Tab, 0);
        imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
        imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
        imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
        imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
        imgui.set_imgui_key(ImGuiKey::PageUp, 5);
        imgui.set_imgui_key(ImGuiKey::PageDown, 6);
        imgui.set_imgui_key(ImGuiKey::Home, 7);
        imgui.set_imgui_key(ImGuiKey::End, 8);
        imgui.set_imgui_key(ImGuiKey::Delete, 9);
        imgui.set_imgui_key(ImGuiKey::Backspace, 10);
        imgui.set_imgui_key(ImGuiKey::Enter, 11);
        imgui.set_imgui_key(ImGuiKey::Escape, 12);
        imgui.set_imgui_key(ImGuiKey::A, 13);
        imgui.set_imgui_key(ImGuiKey::C, 14);
        imgui.set_imgui_key(ImGuiKey::V, 15);
        imgui.set_imgui_key(ImGuiKey::X, 16);
        imgui.set_imgui_key(ImGuiKey::Y, 17);
        imgui.set_imgui_key(ImGuiKey::Z, 18);


        let mouse_state = MouseState::default();

        GUI {
            display: display,
            events_loop: events_loop,
            imgui: imgui,
            renderer: renderer,
            last_frame: Instant::now(),
            mouse_state: mouse_state,
            show_imgui: true,
        }
    }

    fn update_mouse(&mut self) {
        let scale = self.imgui.display_framebuffer_scale();
        self.imgui.set_mouse_pos(
            self.mouse_state.pos.0 as f32 / scale.0,
            self.mouse_state.pos.1 as f32 / scale.1,
        );
        self.imgui.set_mouse_down(
            &[
                self.mouse_state.pressed.0,
                self.mouse_state.pressed.1,
                self.mouse_state.pressed.2,
                false,
                false,
            ],
        );
        self.imgui.set_mouse_wheel(self.mouse_state.wheel / scale.1);
        self.mouse_state.wheel = 0.0;
    }

    pub fn render<F: FnMut(&Ui, &mut Lameboy)>(&mut self, clear_color: (f32, f32, f32, f32), mut lameboy: &mut Lameboy, mut run_ui: F) {
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        self.update_mouse();

        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1, clear_color.2, clear_color.3);

        lameboy.get_ppu().draw(&mut target);

        let window = self.display.gl_window();
        let size_pixels = window.get_inner_size().unwrap();
        let hdipi = window.hidpi_factor();
        let size_points = (
            (size_pixels.0 as f32 / hdipi) as u32,
            (size_pixels.1 as f32 / hdipi) as u32,
        );

        if self.show_imgui {
            let ui = self.imgui.frame(size_points, size_pixels, delta_s);

            run_ui(&ui, &mut lameboy);

            self.renderer.render(&mut target, ui).expect("Rendering failed");
        }

        target.finish().unwrap();
    }

    pub fn update_events(&mut self, gui_state: &mut ImguiDebug, lameboy: &mut Lameboy) -> () {
        use glium::glutin::WindowEvent::*;
        use glium::glutin::ElementState::Pressed;
        use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode};

        let im = &mut self.imgui;
        let mouse = &mut self.mouse_state;

        self.events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    Closed => gui_state.active = false,
                    KeyboardInput { input, .. } => {
                        let pressed = input.state == Pressed;
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Tab) => im.set_key(0, pressed),
                            Some(VirtualKeyCode::Left) => {
                                im.set_key(1, pressed);
                                lameboy.get_joypad().left = pressed
                            }
                            Some(VirtualKeyCode::Right) => {
                                im.set_key(2, pressed);
                                lameboy.get_joypad().right = pressed
                            }
                            Some(VirtualKeyCode::Up) => {
                                im.set_key(3, pressed);
                                lameboy.get_joypad().up = pressed
                            }
                            Some(VirtualKeyCode::Down) => {
                                im.set_key(4, pressed);
                                lameboy.get_joypad().down = pressed
                            }
                            Some(VirtualKeyCode::PageUp) => im.set_key(5, pressed),
                            Some(VirtualKeyCode::PageDown) => im.set_key(6, pressed),
                            Some(VirtualKeyCode::Home) => im.set_key(7, pressed),
                            Some(VirtualKeyCode::End) => im.set_key(8, pressed),
                            Some(VirtualKeyCode::Delete) => im.set_key(9, pressed),
                            Some(VirtualKeyCode::Back) => im.set_key(10, pressed),
                            Some(VirtualKeyCode::Return) => {
                                im.set_key(11, pressed);
                                lameboy.get_joypad().start = pressed
                            }
                            Some(VirtualKeyCode::Escape) => im.set_key(12, pressed),
                            Some(VirtualKeyCode::A) => {
                                im.set_key(13, pressed);
                                lameboy.get_joypad().a = pressed
                            }
                            Some(VirtualKeyCode::S) => { lameboy.get_joypad().b = pressed }
                            Some(VirtualKeyCode::C) => im.set_key(14, pressed),
                            Some(VirtualKeyCode::V) => im.set_key(15, pressed),
                            Some(VirtualKeyCode::X) => im.set_key(16, pressed),
                            Some(VirtualKeyCode::Y) => im.set_key(17, pressed),
                            Some(VirtualKeyCode::Z) => im.set_key(18, pressed),
                            Some(VirtualKeyCode::Space) => im.set_key(20, pressed),
                            Some(VirtualKeyCode::LControl) |
                            Some(VirtualKeyCode::RControl) => im.set_key_ctrl(pressed),
                            Some(VirtualKeyCode::LShift) |
                            Some(VirtualKeyCode::RShift) => {
                                im.set_key_shift(pressed);
                                lameboy.get_joypad().select = pressed
                            }
                            Some(VirtualKeyCode::LAlt) |
                            Some(VirtualKeyCode::RAlt) => im.set_key_alt(pressed),
                            Some(VirtualKeyCode::LWin) |
                            Some(VirtualKeyCode::RWin) => im.set_key_super(pressed),
                            _ => {}
                        }
                    }

                    CursorMoved { position: (x, y), .. } => mouse.pos = (x as i32, y as i32),
                    MouseInput { state, button, .. } => {
                        match button {
                            MouseButton::Left => mouse.pressed.0 = state == Pressed,
                            MouseButton::Right => mouse.pressed.1 = state == Pressed,
                            MouseButton::Middle => mouse.pressed.2 = state == Pressed,
                            _ => {}
                        }
                    }
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } |
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse.wheel = y,
                    CursorEntered { .. } => gui_state.show_menu = true,
                    CursorLeft { .. } => gui_state.show_menu = false,
                    ReceivedCharacter(c) => im.add_input_character(c),
                    _ => (),
                }
            }
        })
    }
}
