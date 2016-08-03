
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

pub fn back() -> Vec<Vertex> {

    return vec![
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [1.0, -1.0] },
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [-1.0, 1.0] },
    ];

}

impl AThing {
    pub fn new() -> AThing {
        return AThing {
            speed: 0.0005,
            position: -0.5,
            time: 0.0,
        };
    }

    pub fn shape(&self) -> Vec<Vertex> {
        return vec![
            Vertex { position: [-0.75, 0.0] },
            Vertex { position: [-0.7, 0.0] },
            Vertex { position: [-0.65, 0.0] },
            Vertex { position: [-0.6, 0.0] },
            Vertex { position: [-0.55, 0.0] },
            Vertex { position: [-0.5, 0.0] },
            Vertex { position: [-0.45, 0.0] },
            Vertex { position: [-0.4, 0.0] },
            Vertex { position: [-0.35, 0.0] },
            Vertex { position: [-0.3, 0.0] },
            Vertex { position: [-0.25, 0.0] },
            Vertex { position: [-0.2, 0.0] },
            Vertex { position: [-0.15, 0.0] },
            Vertex { position: [-0.1, 0.0] },
            Vertex { position: [-0.05, 0.0] },
            Vertex { position: [0.0, 0.0] },
            Vertex { position: [0.05, 0.0] },
            Vertex { position: [0.1, 0.0] },
            Vertex { position: [0.15, 0.0] },
            Vertex { position: [0.2, 0.0] },
            Vertex { position: [0.25, 0.0] },
            Vertex { position: [0.3, 0.0] },
            Vertex { position: [0.35, 0.0] },
            Vertex { position: [0.4, 0.0] },
            Vertex { position: [0.45, 0.0] },
            Vertex { position: [0.5, 0.0] },
            Vertex { position: [0.55, 0.0] },
            Vertex { position: [0.6, 0.0] },
            Vertex { position: [0.65, 0.0] },
            Vertex { position: [0.7, 0.0] },
            Vertex { position: [0.75, 0.0] },
        ];
    }

    pub fn next(&self) -> AThing {

        if self.position > 0.5 || self.position < -0.5 {
            return AThing {
                speed: -self.speed,
                position: self.position - self.speed,
                time: self.time + 0.1,
            };
        } else {
            return AThing {
                speed: self.speed,
                position: self.position + self.speed,
                time: self.time + 0.1,
            };
        }

    }
}
