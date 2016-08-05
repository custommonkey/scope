

use glium::Program;
use glium::framebuffer::SimpleFrameBuffer;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;
use glium::index::NoIndices;
use glium::DrawParameters;
use glium::Surface;

pub struct Channel {
    pub speed: f32,
    pub position: f32,
    pub time: f32,
    pub vertex: VertexBuffer<Vertex>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Channel {
    pub fn new(y: f32, display: &GlutinFacade) -> Channel {
        Channel {
            speed: 0.0005,
            position: -0.5,
            time: 0.0,
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

    pub fn draw(&self,
                framebuffer: &mut SimpleFrameBuffer,
                indices: &NoIndices,
                program: &Program,
                params: &DrawParameters) {

        framebuffer.draw(&self.vertex,
                  indices,
                  &program,
                  &uniform! {t: self.position, time: self.time },
                  &params)
            .unwrap();
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
