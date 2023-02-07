use std::time::Instant;

use glium::glutin;
use glium::glutin::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontSource, Style};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use lameboy::Lameboy;

pub mod imgui_debug_state;
pub mod imgui_debuggable;

pub struct Gui {
    event_loop: EventLoop<()>,
    pub display: glium::Display,
    imgui: Context,
    platform: WinitPlatform,
    renderer: Renderer,
    last_frame: Instant,
    background_colour: (f32, f32, f32, f32),
}

impl Gui {
    pub fn init(
        window_size: (f64, f64),
        window_title: String,
        background_colour: (f32, f32, f32, f32),
    ) -> Gui {
        let events_loop = EventLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let builder = WindowBuilder::new()
            .with_title(window_title)
            .with_inner_size(glutin::dpi::LogicalSize::new(window_size.0, window_size.1));
        let display =
            Display::new(builder, context, &events_loop).expect("Failed to initialize display");

        let mut imgui = Context::create();
        imgui.style_mut().window_rounding = 0.;
        imgui.set_ini_filename(Some(std::path::PathBuf::from("lameboy.ini")));

        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
        }

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        }]);

        imgui.io_mut().font_global_scale = (1.0 / &hidpi_factor) as f32;

        let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

        Gui {
            event_loop: events_loop,
            display,
            imgui,
            platform,
            renderer,
            last_frame: Instant::now(),
            background_colour,
        }
    }

    pub fn main_loop(self, mut lameboy: Lameboy) {
        let Gui {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            background_colour,
            ..
        } = self;
        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(
                    background_colour.0,
                    background_colour.1,
                    background_colour.2,
                    background_colour.3,
                );
                platform.prepare_render(&ui, gl_window.window());

                if lameboy.is_running() {
                    lameboy.run_frame();
                }

                lameboy.get_ppu().draw(&mut target);
                lameboy.imgui_display(&ui);

                if !lameboy.active {
                    *control_flow = ControlFlow::Exit;
                }

                let draw_data = ui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);

                if let Event::WindowEvent { event: i, .. } = event {
                    Gui::update_events(&mut lameboy, &i, imgui.style_mut());
                }
            }
        })
    }

    fn update_events(lameboy: &mut Lameboy, event: &WindowEvent, style: &mut Style) {
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
            WindowEvent::CursorEntered { .. } => {
                lameboy.debug.show_menu = true;
                style.alpha = 0.95;
            }
            WindowEvent::CursorLeft { .. } => {
                lameboy.debug.show_menu = false;
                style.alpha = 0.75;
            }
            _ => (),
        };
    }
}
