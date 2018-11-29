use glium;
use glium::backend::Facade;
use glium::index::PrimitiveType;
use glium::texture::pixel_buffer::PixelBuffer;
use glium::texture::texture2d::Texture2d;
use glium::uniforms::*;
use glium::{IndexBuffer, Program, Surface, VertexBuffer};
use nalgebra::Matrix4;

use ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

const VERTEX_SHADER_SRC: &str = r#"
            #version 140
            in vec2 position;
            in vec2 tex_coords;

            out vec2 v_tex_coords;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
const FRAGMENT_SHADER_SRC: &str = r#"
            #version 140

            uniform sampler2D tex;
            uniform mat4 palette;

            in vec2 v_tex_coords;

            out vec3 color;

            void main() {
                float palette_index = texture(tex, v_tex_coords).x;
                vec4 paletted_color = palette[uint(palette_index * 255.0 + 0.5)];

                float gamma = 2.2;
                vec3 diffuseColor = pow(paletted_color.rgb, vec3(gamma));
                color = diffuseColor;
            }
        "#;

pub struct GPU {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
    texture: Texture2d,
    palette: Matrix4<f32>,
    pixel_buffer: PixelBuffer<u8>,
}

impl GPU {
    pub fn new<F: Facade>(display: &F) -> GPU {
        let vertexes = [
            Vertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0],
                tex_coords: [1.0, 1.0],
            },
        ];

        let vertex_buffer = match VertexBuffer::immutable(display, &vertexes) {
            Ok(vb) => vb,
            Err(e) => panic!("Failed to create OpenGL Vertex Buffer {}", e),
        };
        let index_buffer =
            match IndexBuffer::immutable(display, PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3]) {
                Ok(ib) => ib,
                Err(e) => panic!("Failed to create OpenGL Index Buffer {}", e),
            };

        let program = match glium::Program::from_source(
            display,
            VERTEX_SHADER_SRC,
            FRAGMENT_SHADER_SRC,
            None,
        ) {
            Ok(p) => p,
            Err(e) => panic!("Failed to create OpenGL Program {}", e),
        };

        let pixel_buffer = PixelBuffer::new_empty(display, SCREEN_WIDTH * SCREEN_HEIGHT);
        let empty_pixel_buffer = &vec![0 as u8; pixel_buffer.get_size()];
        pixel_buffer.write(empty_pixel_buffer);

        let texture = match glium::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap,
            160,
            144,
        ) {
            Ok(t) => t,
            Err(e) => panic!("Failed to create OpenGL Texture {}", e),
        };

        // Initialise texture with empty buffer
        texture.main_level().raw_upload_from_pixel_buffer(
            pixel_buffer.as_slice(),
            0..160,
            0..144,
            0..1,
        );

        let palette = Matrix4::new(
            224.0, 136.0, 52.0, 8.0, 248.0, 192.0, 104.0, 24.0, 208.0, 112.0, 86.0, 32.0, 1.0, 1.0,
            1.0, 1.0,
        ) / 255.0;

        GPU {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
            texture: texture,
            palette: palette,
            pixel_buffer: pixel_buffer,
        }
    }

    pub fn draw<S: Surface>(&self, target: &mut S) {
        let palette: &[[f32; 4]; 4] = self.palette.as_ref();
        let uniforms = uniform! {
            palette: palette.clone(),
            tex: self.texture.sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Nearest),
        };

        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            ).unwrap();
    }

    /// Fill the texture with data
    pub fn load_texture(&mut self, image: &Vec<u8>) {
        // Load image pixels into pixel buffer
        self.pixel_buffer.write(&image);
        // Load texture with data from pixel buffer
        self.texture.main_level().raw_upload_from_pixel_buffer(
            self.pixel_buffer.as_slice(),
            0..160,
            0..144,
            0..1,
        );
    }
}
