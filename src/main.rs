#[macro_use]

extern crate glium;

mod thing;

//use glium::glutin;
use glium::BlitTarget;
use glium::DisplayBuild;
use glium::DrawParameters;
use glium::PolygonMode;
use glium::Program;
use glium::Rect;
use glium::Surface;
use glium::VertexBuffer;
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

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform float t;
        uniform float time;

        void main() {
            vec2 pos = position;
            pos.y += noise1(time + pos.x) / 2;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let vertex_blur = r#"
        #version 140

        in vec2 position;

        void main() {
            vec2 pos = position;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src1 = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let fragment_blur = r#"
        #version 140

        out vec4 color;

        uniform sampler2D fb;

		vec2 iResolution = vec2(1024, 768);

        float normpdf(in float x, in float sigma) {
			return 0.39894 * exp(-0.5 * x * x / (sigma * sigma)) / sigma;
		}

        const int mSize = 11;
        const int kSize = (mSize-1)/2;
        const float sigma = 7.0;


        void main() {

	        vec3 c = texture(fb, gl_FragCoord.xy).rgb;


	    	//declare stuff
			vec3 final_colour = vec3(0.0);

			//create the 1-D kernel
			float Z = 0.0;

			float kernel[mSize];
			for (int j = 0; j <= kSize; ++j) {
				kernel[kSize+j] = kernel[kSize-j] = normpdf(float(j), sigma);
			}


			//get the normalization factor (as the gaussian has been clamped)
			for (int j = 0; j < mSize; ++j) {
				Z += kernel[j];
			}

            vec3 back = vec3(0.9, 0.9, 0.9);

			//read out the texels
			for (int i =- kSize; i <= kSize; ++i) {
				for (int j =- kSize; j <= kSize; ++j) {
					final_colour += kernel[kSize+j]
                        * kernel[kSize + i]
                        * texture(fb, (gl_FragCoord.xy + vec2(float(i), float(j))) / iResolution.xy).rgb
                        * back
                        ;
				}
			}


			color = vec4(final_colour/(Z*Z), 1.0);

        }
    "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src1, None)
        .unwrap();

    let blur_program = Program::from_source(&display, vertex_blur, fragment_blur, None).unwrap();

    let mut a_thing = thing::AThing::new();

    let indices = NoIndices(PrimitiveType::LineStrip);

    let blur_indices = NoIndices(PrimitiveType::TrianglesList);

    let vertex_buffer = VertexBuffer::new(&display, &a_thing.shape()).unwrap();

    let blur_vertex_buffer = VertexBuffer::new(&display, &thing::back()).unwrap();

    let texture = Texture2d::empty_with_format(&display,
                                               UncompressedFloatFormat::U8U8U8,
                                               MipmapsOption::NoMipmap,
                                               src.width,
                                               src.height)
        .unwrap();


    let mut framebuffer = SimpleFrameBuffer::new(&display, &texture).unwrap();

    framebuffer.clear_color(0.92, 0.91, 0.81, 1.0);

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
