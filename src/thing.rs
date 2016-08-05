use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;

pub struct AThing {
    pub speed: f32,
    pub position: f32,
    pub time: f32,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl AThing {
    pub fn new() -> AThing {
        return AThing {
            speed: 0.0005,
            position: -0.5,
            time: 0.0,
        };
    }

    pub fn buffer(&self, display: &GlutinFacade) -> VertexBuffer<Vertex> {

        let vertices: Vec<Vertex> =
            (-90..91).map(|n| Vertex { position: [(n as f32) / 100.0, 0.0] }).collect();

        return VertexBuffer::new(display, &vertices).unwrap();
    }

    pub fn back_buffer(&self, display: &GlutinFacade) -> VertexBuffer<Vertex> {

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

        return VertexBuffer::new(display, &back).unwrap();
    }

    pub fn next(&self) -> AThing {

        if self.position > 0.5 || self.position < -0.5 {
            return AThing {
                speed: -self.speed,
                position: self.position - self.speed,
                time: self.time + 0.005,
            };
        } else {
            return AThing {
                speed: self.speed,
                position: self.position + self.speed,
                time: self.time + 0.005,
            };
        }

    }
}
