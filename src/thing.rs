
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

    let x = 1.0;
    let y = 0.5;

    return vec![
        Vertex { position: [-x, -y] },
        Vertex { position: [x, y] },
        Vertex { position: [x, -y] },
        Vertex { position: [-x, -y] },
        Vertex { position: [x, y] },
        Vertex { position: [-x, y] },
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

        let mut v = Vec::new();

        for n in -90..91 {
            let f = (n as f32) / 100.0;
            v.push(Vertex { position: [f, 0.0] });
        }

        return v;
    }

    pub fn next(&self) -> AThing {

        if self.position > 0.5 || self.position < -0.5 {
            return AThing {
                speed: -self.speed,
                position: self.position - self.speed,
                time: self.time + 0.2,
            };
        } else {
            return AThing {
                speed: self.speed,
                position: self.position + self.speed,
                time: self.time + 0.2,
            };
        }

    }
}
