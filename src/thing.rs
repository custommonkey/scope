


pub struct AThing {
    pub speed: f32,
    pub position: f32,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

impl AThing {
    pub fn new() -> AThing {
        return AThing {
            speed: 0.0005,
            position: -0.5,
        };
    }

    pub fn shape(&self) -> Vec<Vertex> {

        implement_vertex!(Vertex, position);

        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [0.0, 0.5] };
        let vertex3 = Vertex { position: [0.5, -0.25] };

        return vec![vertex1, vertex2, vertex3];
    }

    pub fn next(&self) -> AThing {

        if self.position > 0.5 || self.position < -0.5 {
            return AThing {
                speed: -self.speed,
                position: self.position - self.speed,
            };
        } else {
            return AThing {
                speed: self.speed,
                position: self.position + self.speed,
            };
        }

    }
}
