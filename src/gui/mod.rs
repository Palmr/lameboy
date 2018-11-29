use glium::glutin;
use glium::glutin::EventsLoop;
use glium::{Display, Surface};
use imgui::{FrameSize, ImGui, ImGuiKey, ImString, Ui};
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
    hidpi_factor: f64,
    events_loop: EventsLoop,
    imgui: ImGui,
    renderer: Renderer,
    last_frame: Instant,
    mouse_state: MouseState,
    show_imgui: bool,
}

impl GUI {
    pub fn init(window_size: (f64, f64), rom_file_name: &str) -> GUI {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let window = glutin::WindowBuilder::new()
            .with_title(format!("{} - Lameboy - v0.1", rom_file_name))
            .with_dimensions(glutin::dpi::LogicalSize::new(window_size.0, window_size.1));
        let display = Display::new(window, context, &events_loop).unwrap();
        let hidpi_factor = display.gl_window().get_hidpi_factor();

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
            display,
            hidpi_factor,
            events_loop,
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state,
            show_imgui: true,
        }
    }

    fn update_mouse(&mut self) {
        let scale = self.imgui.display_framebuffer_scale();
        self.imgui.set_mouse_pos(
            self.mouse_state.pos.0 as f32 / scale.0,
            self.mouse_state.pos.1 as f32 / scale.1,
        );
        self.imgui.set_mouse_down([
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ]);
        self.imgui.set_mouse_wheel(self.mouse_state.wheel / scale.1);
        self.mouse_state.wheel = 0.0;
    }

    pub fn render<F: FnMut(&Ui, &mut Lameboy)>(
        &mut self,
        clear_color: (f32, f32, f32, f32),
        mut lameboy: &mut Lameboy,
        mut run_ui: F,
    ) {
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        self.update_mouse();

        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1, clear_color.2, clear_color.3);

        lameboy.get_ppu().draw(&mut target);

        let window = self.display.gl_window();
        let physical_size = window
            .get_inner_size()
            .unwrap()
            .to_physical(window.get_hidpi_factor());
        let logical_size = physical_size.to_logical(self.hidpi_factor);

        let frame_size = FrameSize {
            logical_size: logical_size.into(),
            hidpi_factor: self.hidpi_factor,
        };

        if self.show_imgui {
            let ui = self.imgui.frame(frame_size, delta_s);

            run_ui(&ui, &mut lameboy);

            self.renderer
                .render(&mut target, ui)
                .expect("Rendering failed");
        }

        target.finish().unwrap();
    }

    pub fn update_events(&mut self, gui_state: &mut ImguiDebug, lameboy: &mut Lameboy) -> () {
        let im = &mut self.imgui;
        let mouse = &mut self.mouse_state;
        let hidpi_factor = &self.hidpi_factor;

        self.events_loop.poll_events(|event| {
            use glium::glutin::ElementState::Pressed;
            use glium::glutin::WindowEvent::*;
            use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    CloseRequested => gui_state.active = false,
                    KeyboardInput { input, .. } => {
                        use glium::glutin::VirtualKeyCode as Key;

                        let pressed = input.state == Pressed;
                        match input.virtual_keycode {
                            Some(Key::Tab) => im.set_key(0, pressed),
                            Some(Key::Left) => {
                                im.set_key(1, pressed);
                                lameboy.get_joypad().left = pressed
                            }
                            Some(Key::Right) => {
                                im.set_key(2, pressed);
                                lameboy.get_joypad().right = pressed
                            }
                            Some(Key::Up) => {
                                im.set_key(3, pressed);
                                lameboy.get_joypad().up = pressed
                            }
                            Some(Key::Down) => {
                                im.set_key(4, pressed);
                                lameboy.get_joypad().down = pressed
                            }
                            Some(Key::PageUp) => im.set_key(5, pressed),
                            Some(Key::PageDown) => im.set_key(6, pressed),
                            Some(Key::Home) => im.set_key(7, pressed),
                            Some(Key::End) => im.set_key(8, pressed),
                            Some(Key::Delete) => im.set_key(9, pressed),
                            Some(Key::Back) => im.set_key(10, pressed),
                            Some(Key::Return) => {
                                im.set_key(11, pressed);
                                lameboy.get_joypad().start = pressed
                            }
                            Some(Key::Escape) => im.set_key(12, pressed),
                            Some(Key::A) => {
                                im.set_key(13, pressed);
                                lameboy.get_joypad().a = pressed
                            }
                            Some(Key::S) => lameboy.get_joypad().b = pressed,
                            Some(Key::C) => im.set_key(14, pressed),
                            Some(Key::V) => im.set_key(15, pressed),
                            Some(Key::X) => im.set_key(16, pressed),
                            Some(Key::Y) => im.set_key(17, pressed),
                            Some(Key::Z) => im.set_key(18, pressed),
                            Some(Key::LControl) | Some(Key::RControl) => im.set_key_ctrl(pressed),
                            Some(Key::LShift) | Some(Key::RShift) => {
                                im.set_key_shift(pressed);
                                lameboy.get_joypad().select = pressed
                            }
                            Some(Key::LAlt) | Some(Key::RAlt) => im.set_key_alt(pressed),
                            Some(Key::LWin) | Some(Key::RWin) => im.set_key_super(pressed),
                            _ => {}
                        }
                    }
                    CursorMoved { position: pos, .. } => {
                        // Rescale position from glutin logical coordinates to our logical
                        // coordinates
                        mouse.pos = pos
                            .to_physical(hidpi_factor.clone())
                            .to_logical(hidpi_factor.round())
                            .into();
                    }
                    CursorEntered { .. } => gui_state.show_menu = true,
                    CursorLeft { .. } => gui_state.show_menu = false,
                    MouseInput { state, button, .. } => match button {
                        MouseButton::Left => mouse.pressed.0 = state == Pressed,
                        MouseButton::Right => mouse.pressed.1 = state == Pressed,
                        MouseButton::Middle => mouse.pressed.2 = state == Pressed,
                        _ => {}
                    },
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse.wheel = y,
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(pos),
                        phase: TouchPhase::Moved,
                        ..
                    } => {
                        // Rescale pixel delta from glutin logical coordinates to our logical
                        // coordinates
                        mouse.wheel = pos
                            .to_physical(hidpi_factor.clone())
                            .to_logical(hidpi_factor.round())
                            .y as f32;
                    }
                    ReceivedCharacter(c) => im.add_input_character(c),
                    _ => (),
                }
            }
        })
    }
}
