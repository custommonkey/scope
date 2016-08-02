

pub struct AThing {
    pub speed: f32,
    pub position: f32,
}

impl AThing {
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
