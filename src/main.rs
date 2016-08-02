#[macro_use]

extern crate glium;

mod thing;

fn main() {

    use glium::DisplayBuild;
    use glium::Surface;

    let params = glium::DrawParameters {
        point_size: Some(10.0),
        polygon_mode: glium::PolygonMode::Line,
        multisampling: true, // Why isn't this having any effect
        ..Default::default()
    };

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
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
            pos.y += noise1((pos.y +t)*10);
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.48, 0.31, 0.22, 1.0);
        }
    "#;

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let mut a_thing = thing::AThing {
        speed: 0.0005,
        position: -0.5,
    };

    loop {

        a_thing = a_thing.next();

        let mut target = display.draw();

        target.clear_color(0.92, 0.91, 0.81, 1.0);

        target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &uniform! {t: a_thing.position},
                  &params)
            .unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }

}
