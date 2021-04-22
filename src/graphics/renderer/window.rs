use glium;
use glium::{ glutin, Surface };

use super::vertex::Vertex;

pub struct Window<'a> {
    title: &'a str,
    dimensions: (f32, f32),
    event_loop: glutin::event_loop::EventLoop,
    display: glium::Display,
}

impl<'a> Window<'a> {
    fn new(title: &'a str, dimensions: (f32, f32)) -> Window<'a> {
        let mut event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(dimensions.0, dimensions.1))
            .with_title(title);
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        Window {
            title,
            dimensions,
            event_loop,
            display,
        }
    }

    fn run(&self) {
        let vertex1 = Vertex { position: [ -0.5, -0.5  ] };
        let vertex2 = Vertex { position: [  0.0,  0.5  ] };
        let vertex3 = Vertex { position: [  0.5, -0.25 ] };
        let shape = vec![vertex1, vertex2, vertex3];
    
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

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        self.event_loop.run(move |ev, _, control_flow| {
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
    
            target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    
            target.finish().unwrap();
    
            let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                _ => (),
            }
        });
    }
}
