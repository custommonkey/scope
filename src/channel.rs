
use glium::PolygonMode;

use glium::Program;
use glium::framebuffer::SimpleFrameBuffer;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;

use glium::DrawParameters;
use glium::Surface;
use glium::index::NoIndices;
use glium::index::PrimitiveType;

pub struct Channel<'a> {
    pub speed: f32,
    pub position: f32,
    pub time: f32,
    vertex: VertexBuffer<Vertex>,
    indices: &'a NoIndices,
    program: &'a Program,
    params: &'a DrawParameters<'a>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl<'a> Channel<'a> {
    fn new(y: f32,
           display: &GlutinFacade,
           indices: &'a NoIndices,
           program: &'a Program,
           params: &'a DrawParameters)
           -> Channel<'a> {
        Channel {
            speed: 0.0005,
            position: -0.5,
            time: 0.0,
            indices: indices,
            program: program,
            params: params,
            vertex: {
                let vertices: Vec<Vertex> = (-90..91)
                    .map(|n| Vertex { position: [(n as f32) / 100.0, y as f32] })
                    .collect();

                VertexBuffer::new(display, &vertices).unwrap()
            },
        }
    }


    pub fn next(&mut self) {

        self.time = self.time + 0.005;

        if self.position > 0.5 || self.position < -0.5 {

            self.speed = -self.speed;
            self.position = self.position - self.speed;

        } else {

            self.position = self.position + self.speed;

        }

    }

    pub fn draw(&self, framebuffer: &mut SimpleFrameBuffer) {

        framebuffer.draw(&self.vertex,
                  self.indices,
                  &self.program,
                  &uniform! {t: self.position, time: self.time },
                  &self.params)
            .unwrap();
    }
}

pub struct ChannelFactory<'a> {
    display: &'a GlutinFacade,
    indices: NoIndices,
    program: &'a Program,
    params: DrawParameters<'a>,
}

impl<'a> ChannelFactory<'a> {
    pub fn new(display: &'a GlutinFacade, program: &'a Program) -> ChannelFactory<'a> {

        let params = DrawParameters {
            point_size: Some(2.0),
            line_width: Some(10.0),
            polygon_mode: PolygonMode::Line,
            multisampling: false, // Why isn't this having any effect
            ..Default::default()
        };

        let indices = NoIndices(PrimitiveType::LineStrip);

        ChannelFactory {
            display: display,
            indices: indices,
            program: program,
            params: params,
        }
    }

    pub fn channel(&self, y: f32) -> Channel {
        Channel::new(y, self.display, &self.indices, self.program, &self.params)
    }
}

pub fn back_buffer(display: &GlutinFacade) -> VertexBuffer<Vertex> {

    let x = 1.0;
    let y = 0.5;

    let x1 = -x;
    let x2 = x;
    let y1 = -y;
    let y2 = y;

    let back = vec![
	        Vertex { position: [x1, y1] },
	        Vertex { position: [x2, y1] },
	        Vertex { position: [x2, y2] },
	        Vertex { position: [x1, y1] },
	        Vertex { position: [x1, y2] },
	        Vertex { position: [x2, y2] },
	    ];

    VertexBuffer::new(display, &back).unwrap()
}
