#[macro_use]

extern crate glium;

mod thing;
mod filter;

use filter::Filter;
use glium::BlitTarget;
use glium::DisplayBuild;
use glium::DrawParameters;
use glium::PolygonMode;
use glium::Rect;
use glium::Surface;
use glium::framebuffer::SimpleFrameBuffer;
use glium::glutin::Event;
use glium::glutin::WindowBuilder;
use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::texture::MipmapsOption;
use glium::texture::Texture2d;
use glium::texture::UncompressedFloatFormat;
use glium::uniforms::MagnifySamplerFilter;

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
        width: src.width as i32,
        height: src.height as i32,
    };

    let params = DrawParameters {
        point_size: Some(10.0),
        line_width: Some(10.0),
        polygon_mode: PolygonMode::Line,
        multisampling: true, // Why isn't this having any effect
        ..Default::default()
    };

    let blur_params = DrawParameters {
        point_size: Some(10.0),
        line_width: Some(10.0),
        polygon_mode: PolygonMode::Fill,
        multisampling: true, // Why isn't this having any effect
        ..Default::default()
    };

    let display = WindowBuilder::new()
        //.with_fullscreen(glutin::get_primary_monitor())
        .with_dimensions(src.width, src.height)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    let f1 = Filter::new("vertex.glsl", "fragment.glsl");

    let blur = Filter::new("vertex_blur.glsl", "fragment_blur.glsl");

    // let blur1 = Filter::new("HBlurVertexShader.glsl", "BlurFragmentShader.glsl");
    // let blur2 = Filter::new("VBlurVertexShader.glsl", "BlurFragmentShader.glsl");

    let program = f1.program(&display);

    let blur_program = blur.program(&display);

    let mut a_thing = thing::AThing::new();

    let indices = NoIndices(PrimitiveType::LineStrip);

    let blur_indices = NoIndices(PrimitiveType::TrianglesList);

    let vertex_buffer = a_thing.buffer(&display);

    let blur_vertex_buffer = a_thing.back_buffer(&display);

    let texture = Texture2d::empty_with_format(&display,
                                               UncompressedFloatFormat::U8U8U8,
                                               MipmapsOption::NoMipmap,
                                               src.width,
                                               src.height)
        .unwrap();


    let mut framebuffer = SimpleFrameBuffer::new(&display, &texture).unwrap();

    framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);

    loop {

        framebuffer.draw(&blur_vertex_buffer,
                  &blur_indices,
                  &blur_program,
                  &uniform! { fb: &texture },
                  &blur_params)
            .unwrap();

        a_thing = a_thing.next();

        framebuffer.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &uniform! {t: a_thing.position, time: a_thing.time },
                  &params)
            .unwrap();

        let target = display.draw();

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
