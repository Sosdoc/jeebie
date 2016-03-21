use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use glium::Display;
use glium::texture::{RawImage2d, Texture2d, ClientFormat};
use glium::{glutin, index, uniforms};


use std::borrow::Cow;

use jeebie::frontend::GpuFrontend;

// TODO: this is pretty much the tutorial for glium
// some stuff that needs to be done:
//  - make the framebuffer data a struct with raw data and resolution
//  - define a trait for input handling and make GliumFrontend implement it

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub struct GliumFrontend {
    display: Box<Display>,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: index::NoIndices,
    program: Program,

    pub should_run: bool,
    texture: Texture2d,
    tex_size: (u32, u32),
}

impl GliumFrontend {
    pub fn new_with_size(size: (u32, u32)) -> GliumFrontend {
        let display = glutin::WindowBuilder::new().build_glium().unwrap();

        implement_vertex!(Vertex, position, tex_coords);
        let vertex1 = Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        };
        let vertex2 = Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, 0.0],
        };
        let vertex3 = Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        };
        let vertex4 = Vertex {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 1.0],
        };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
        let indices = index::NoIndices(index::PrimitiveType::TriangleFan);

        let vertex_shader_src = r#"
           #version 140
           in vec2 position;
           in vec2 tex_coords;
           out vec2 v_tex_coords;

           void main() {
               v_tex_coords = tex_coords;
               gl_Position = vec4(position, 0.0, 1.0);
           }
        "#;

        let fragment_shader_src = r#"
           #version 140

           in vec2 v_tex_coords;
           out vec4 color;

           uniform sampler2D tex;

           void main() {
               color = texture(tex, v_tex_coords);
           }
        "#;

        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let texture = Texture2d::empty(&display, size.0, size.1).unwrap();

        GliumFrontend {
            display: Box::new(display),
            vertex_buffer: vertex_buffer,
            indices: indices,
            program: program,

            should_run: true,
            tex_size: size,
            texture: texture,
        }
    }

    fn draw(&self, image: RawImage2d<(u8, u8, u8)>) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        //let texture = Texture2d::new(&*self.display, image).unwrap();

        let uniforms = uniform! {
            tex: &self.texture,
        };

        target.draw(&self.vertex_buffer,
                    &self.indices,
                    &self.program,
                    &uniforms::EmptyUniforms,
                    &Default::default())
              .unwrap();
        target.finish().unwrap();
    }

    pub fn update(&mut self) {
        // TODO: this is necessary for input, map keys to actions for the emulator
        for ev in self.display.poll_events() {
            if let glutin::Event::Closed = ev {
                self.should_run = false;
            }
        }
    }
}


impl GpuFrontend for GliumFrontend {
    fn display_frame(&mut self, framebuffer: &[(u8, u8, u8)]) {

        let image = RawImage2d {
            data: Cow::Borrowed(framebuffer), // moo
            width: self.tex_size.0,
            height: self.tex_size.1,
            format: ClientFormat::U8U8U8,
        };
        // TODO: run update somewhere
        self.draw(image);
    }
}
