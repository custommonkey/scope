#[macro_use]

extern crate glium;

mod thing;

use glium::uniforms::MagnifySamplerFilter;
use glium::texture::Texture2d;
use glium::texture::UncompressedFloatFormat;
use glium::texture::MipmapsOption;
use glium::VertexBuffer;
use glium::DisplayBuild;
use glium::DrawParameters;
use glium::PolygonMode;
use glium::Program;
use glium::Surface;
use glium::glutin::Event;
use glium::glutin::WindowBuilder;
use glium::index::NoIndices;
use glium::index::PrimitiveType::TrianglesList;
use glium::Rect;
use glium::BlitTarget;
use glium::framebuffer::SimpleFrameBuffer;



fn main() {

    let src = Rect {
        left: 0,
        bottom: 0,
        width: 1024,
        height: 768,
    };

    let dest = BlitTarget {
        left: 0,
        bottom: 0,
        width: 1024,
        height: 768,
    };

    let params = DrawParameters {
        point_size: Some(10.0),
        polygon_mode: PolygonMode::Line,
        multisampling: true, // Why isn't this having any effect
        ..Default::default()
    };

    let display = WindowBuilder::new()
        .with_dimensions(src.width, src.height)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform float t;

        void main() {
            vec2 pos = position;
            pos.x += t;
            pos.y += noise1((pos.y + t) * 10);
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        
        uniform sampler2D fb;

        void main() {
            color = vec4(0.48, 0.31, 0.22, 1.0);
        }
    "#;


    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .unwrap();

    let mut a_thing = thing::AThing::new();

    let indices = NoIndices(TrianglesList);

    let vertex_buffer = VertexBuffer::new(&display, &a_thing.shape()).unwrap();

    let texture = Texture2d::empty_with_format(&display,
                                               UncompressedFloatFormat::U8U8U8,
                                               MipmapsOption::NoMipmap,
                                               src.width,
                                               src.height)
        .unwrap();


    let mut framebuffer = SimpleFrameBuffer::new(&display, &texture).unwrap();

    loop {

        a_thing = a_thing.next();

        let mut target = display.draw();

        framebuffer.clear_color(0.92, 0.91, 0.81, 1.0);

        target.clear_color(0.92, 0.91, 0.81, 1.0);

        framebuffer.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &uniform! {t: a_thing.position, fb: &texture},
                  &params)
            .unwrap();

        target.blit_from_simple_framebuffer(&framebuffer,
                                            &src,
                                            &dest,
                                            MagnifySamplerFilter::Nearest);



        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                _ => (),
            }
        }
    }

}
