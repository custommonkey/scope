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
        polygon_mode: PolygonMode::Fill,
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
            pos.x += t + noise1((pos.x + t) * 50) / 10;
            pos.y += noise1((pos.y + t) * 50) / 10;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src1 = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.48, 0.31, 0.22, 1.0);
        }
    "#;

    let fragment_shader_src2 = r#"
        #version 140

        out vec4 color;

        uniform sampler2D fb;

		vec2 iResolution = vec2(1024, 768);

        float normpdf(in float x, in float sigma) {
			return 0.39894 * exp(-0.5 * x * x / (sigma * sigma)) / sigma;
		}

        void main() {

	        vec3 c = texture(fb, gl_FragCoord.xy).rgb;

	        float r = gl_FragCoord.x / 1024;

	    	//declare stuff
			const int mSize = 11;
			const int kSize = (mSize-1)/2;
			float kernel[mSize];
			vec3 final_colour = vec3(0.0);

			//create the 1-D kernel
			float sigma = 7.0;
			float Z = 0.0;

			for (int j = 0; j <= kSize; ++j) {
				kernel[kSize+j] = kernel[kSize-j] = normpdf(float(j), sigma);
			}

			//get the normalization factor (as the gaussian has been clamped)
			for (int j = 0; j < mSize; ++j) {
				Z += kernel[j];
			}

			//read out the texels
			for (int i=-kSize; i <= kSize; ++i) {
				for (int j=-kSize; j <= kSize; ++j) {
					final_colour += kernel[kSize+j] * kernel[kSize+i] *
						texture(fb, (gl_FragCoord.xy+vec2(float(i),float(j))) / iResolution.xy).rgb;
				}
			}

			color = vec4(final_colour/(Z*Z), 1.0);

        }
    "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src1, None)
        .unwrap();

    let blur_program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src2, None).unwrap();

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

        let target = display.draw();

        framebuffer.clear_color(0.92, 0.91, 0.81, 1.0);


        framebuffer.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &uniform! {t: a_thing.position },
                  &params)
            .unwrap();

        framebuffer.draw(&vertex_buffer,
                  &indices,
                  &blur_program,
                  &uniform! {t: a_thing.position, fb: &texture },
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
