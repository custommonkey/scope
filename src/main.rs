#[macro_use]

extern crate glium;
extern crate image;

mod channel;
mod filter;

use channel::ChannelFactory;
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
use std::time::SystemTime;

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


    let blur_params = DrawParameters {
        point_size: Some(2.0),
        line_width: Some(10.0),
        polygon_mode: PolygonMode::Fill,
        multisampling: false, // Why isn't this having any effect
        ..Default::default()
    };

    let crt_params = DrawParameters {
        point_size: Some(2.0),
        line_width: Some(10.0),
        polygon_mode: PolygonMode::Fill,
        multisampling: false, // Why isn't this having any effect
        //        blend: Blend::alpha_blending(),
        ..Default::default()
    };

    let blur_indices = NoIndices(PrimitiveType::TrianglesList);

    let display = WindowBuilder::new()
        //.with_fullscreen(glutin::get_primary_monitor())
        .with_dimensions(src.width, src.height)
        .with_title(format!("Scope"))
        .build_glium()
        .unwrap();

    let f1 = Filter::new("vertex", "fragment");

    let blur = Filter::new("vertex_blur", "fragment_blur");

    let crt = Filter::new("vertex_blur", "crt");

    // let blur1 = Filter::new("HBlurVertexShader.glsl", "BlurFragmentShader.glsl");
    // let blur2 = Filter::new("VBlurVertexShader.glsl", "BlurFragmentShader.glsl");

    let program = f1.program(&display);

    let blur_program = blur.program(&display);

    let crt_program = crt.program(&display);

    let channel_factory = ChannelFactory::new(&display, &program);

    let mut channel0 = channel_factory.channel(0.0);
    let mut channel1 = channel_factory.channel(0.1);
    let mut channel2 = channel_factory.channel(0.2);
    let mut channel3 = channel_factory.channel(0.3);

    let blur_vertex_buffer = channel::back_buffer(&display);

    let snow = Texture2d::new(&display, load_image()).unwrap();

    let texture = Texture2d::empty_with_format(&display,
                                               UncompressedFloatFormat::U8U8U8,
                                               MipmapsOption::NoMipmap,
                                               src.width,
                                               src.height)
        .unwrap();


    let mut framebuffer = SimpleFrameBuffer::new(&display, &texture).unwrap();

    framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);

    let start_time = SystemTime::now();

    loop {

        let elapsed = start_time.elapsed().unwrap();

        let time = (elapsed.as_secs() as f32) + (elapsed.subsec_nanos() as f32) / 1000000000.0;

        framebuffer.draw(&blur_vertex_buffer,
                  &blur_indices,
                  &blur_program,
                  &uniform! { fb: &texture },
                  &blur_params)
            .unwrap();

        framebuffer.draw(&blur_vertex_buffer,
                  &blur_indices,
                  &crt_program,
                  &uniform! { iChannel0: &texture, iChannel1: &snow, iGlobalTime: time },
                  &crt_params)
            .unwrap();

        for c in vec![&mut channel0, &mut channel1, &mut channel2, &mut channel3] {
            c.draw(&mut framebuffer);
        }

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

fn load_image<'a>() -> glium::texture::RawImage2d<'a, u8> {

    use std::io::Cursor;

    let image = image::load(Cursor::new(&include_bytes!("static.png")[..]), image::PNG)
        .unwrap()
        .to_rgba();

    let image_dimensions = image.dimensions();

    glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)

}
