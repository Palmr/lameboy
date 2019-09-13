use std::time::Instant;

use glium::backend::glutin::glutin::{ElementState, VirtualKeyCode};
use glium::glutin::{self, Event, WindowEvent};
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontSource};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use lameboy::Lameboy;

pub mod imgui_debug_state;
pub mod imgui_debuggable;

pub struct GUI {
    events_loop: glutin::EventsLoop,
    pub display: glium::Display,
    imgui: Context,
    platform: WinitPlatform,
    renderer: Renderer,
    font_size: f32,
    last_frame: Instant,
}

impl GUI {
    pub fn init(window_size: (f64, f64), window_title: String) -> GUI {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let builder = glutin::WindowBuilder::new()
            .with_title(window_title)
            .with_dimensions(glutin::dpi::LogicalSize::new(window_size.0, window_size.1));
        let display =
            Display::new(builder, context, &events_loop).expect("Failed to initialize display");

        let mut imgui = Context::create();
        imgui.set_ini_filename(Some(std::path::PathBuf::from("lameboy.ini")));

        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
        }

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        }]);

        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

        GUI {
            events_loop,
            display,
            imgui,
            platform,
            renderer,
            font_size,
            last_frame: Instant::now(),
        }
    }

    pub fn render(&mut self, clear_color: (f32, f32, f32, f32), lameboy: &mut Lameboy) {
        let gl_window = self.display.gl_window();
        let window = gl_window.window();

        let io = self.imgui.io_mut();
        self.platform
            .prepare_frame(io, &window)
            .expect("Failed to start frame");
        self.last_frame = io.update_delta_time(self.last_frame);

        let ui = self.imgui.frame();
        let mut target = self.display.draw();
        target.clear_color_srgb(clear_color.0, clear_color.1, clear_color.2, clear_color.3);

        lameboy.get_ppu().draw(&mut target);

        //        run_ui(&ui, &mut lameboy);
        lameboy.imgui_display(&ui);

        let draw_data = ui.render();
        self.renderer
            .render(&mut target, draw_data)
            .expect("Rendering failed");

        target.finish().expect("Failed to swap buffers");
    }

    pub fn update_events(&mut self, lameboy: &mut Lameboy) {
        let gl_window = self.display.gl_window();
        let window = gl_window.window();
        let im = &mut self.imgui;
        let platform = &mut self.platform;

        self.events_loop.poll_events(|event| {
            platform.handle_event(im.io_mut(), &window, &event);

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => lameboy.active = false,
                    WindowEvent::KeyboardInput { input, .. } => {
                        let pressed = input.state == ElementState::Pressed;
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Left) => lameboy.get_joypad().left = pressed,
                            Some(VirtualKeyCode::Right) => lameboy.get_joypad().right = pressed,
                            Some(VirtualKeyCode::Up) => lameboy.get_joypad().up = pressed,
                            Some(VirtualKeyCode::Down) => lameboy.get_joypad().down = pressed,

                            Some(VirtualKeyCode::Return) => lameboy.get_joypad().start = pressed,
                            Some(VirtualKeyCode::A) => lameboy.get_joypad().a = pressed,
                            Some(VirtualKeyCode::S) => lameboy.get_joypad().b = pressed,
                            Some(VirtualKeyCode::LShift) | Some(VirtualKeyCode::RShift) => {
                                lameboy.get_joypad().select = pressed
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::CursorEntered { .. } => lameboy.debug.show_menu = true,
                    WindowEvent::CursorLeft { .. } => lameboy.debug.show_menu = false,
                    _ => (),
                }
            }
        });
    }
}
