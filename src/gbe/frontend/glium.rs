use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use glium::backend::glutin_backend::GlutinFacade;
use glium::{glutin, index, uniforms};

use gbe::frontend::GpuFrontend;

// TODO: this is pretty much the tutorial for glium
// some stuff that needs to be done:
//  - draw a proper rectangle
//  - map a texture to the rectangle, it should be bounded to the resolution of the framebuffer
//  - make the framebuffer data a struct with raw data and resolution
//  - define a trait for input handling and make GliumFrontend implement it

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

pub struct GliumFrontend {
    display: GlutinFacade,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: index::NoIndices,
    program: Program,
}

impl GliumFrontend {
    pub fn new() -> GliumFrontend {
        let display = glutin::WindowBuilder::new().build_glium().unwrap();

        implement_vertex!(Vertex, position);
        let vertex1 = Vertex { position: [-1.0, -1.0] };
        let vertex2 = Vertex { position: [1.0, -1.0] };
        let vertex3 = Vertex { position: [1.0, 1.0] };
        let vertex4 = Vertex { position: [-1.0, 1.0] };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
        let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
       #version 140
       in vec2 position;
       void main() {
           gl_Position = vec4(position, 0.0, 1.0);
       }
    "#;

        let fragment_shader_src = r#"
       #version 140
       out vec4 color;
       void main() {
           color = vec4(1.0, 0.0, 0.0, 1.0);
       }
    "#;

        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                          .unwrap();

        GliumFrontend {
            display: display,
            vertex_buffer: vertex_buffer,
            indices: indices,
            program: program,
        }
    }

    fn draw(&self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&self.vertex_buffer,
                    &self.indices,
                    &self.program,
                    &uniforms::EmptyUniforms,
                    &Default::default())
              .unwrap();
        target.finish().unwrap();
    }

    fn update(&self) {
        // TODO: this is necessary for input, map keys to actions for the emulator
        for ev in self.display.poll_events() {
            match ev {
                glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}


impl GpuFrontend for GliumFrontend {
    fn display_frame(&mut self, framebuffer: Vec<u8>) {
        // TODO: implement drawing of a texture
        let _ = framebuffer;
        loop {
            self.draw();
            self.update();
        }
    }
}