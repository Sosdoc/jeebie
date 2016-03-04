use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::{RawImage2d, Texture2d, ClientFormat};
use glium::{glutin, index, uniforms};

use std::borrow::Cow;

use gbe::frontend::GpuFrontend;

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
    display: GlutinFacade,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: index::NoIndices,
    program: Program,

    pub should_run: bool,
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

        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                          .unwrap();

        GliumFrontend {
            display: display,
            vertex_buffer: vertex_buffer,
            indices: indices,
            program: program,

            should_run: true,
            tex_size: (160, 144),
        }
    }

    fn draw(&self, image: RawImage2d<(u8, u8, u8)>) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        //let image = RawImage2d::from_raw_rgba(data, self.tex_size);
        let texture = Texture2d::new(&self.display, image).unwrap();

        let uniforms = uniform! {
            tex: &texture,
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
            match ev {
                glutin::Event::Closed => self.should_run = false,
                _ => (),
            }
        }
    }
}


impl GpuFrontend for GliumFrontend {
    fn display_frame(&mut self, framebuffer: Vec<(u8, u8, u8)>) {

        let image = RawImage2d {
            data: Cow::Borrowed(framebuffer.as_slice()), // moo
            width: self.tex_size.0,
            height: self.tex_size.1,
            format: ClientFormat::U8U8U8,
        };
        // TODO: run update somewhere
        self.draw(image);
    }
}
