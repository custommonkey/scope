use glium::backend::glutin_backend::GlutinFacade;
use glium::Program;

use std::io::prelude::*;

use std::fs::File;

pub struct Filter {
    vertex_shader: String,
    fragment_shader: String,
}

impl Filter {
    pub fn new(v_name: &'static str, f_name: &'static str) -> Filter {
        return Filter {
            vertex_shader: load_shader(v_name),
            fragment_shader: load_shader(f_name),
        };
    }
    pub fn program(&self, display: &GlutinFacade) -> Program {
        return Program::from_source(display, &self.vertex_shader, &self.fragment_shader, None)
            .unwrap();
    }
}

fn load_shader(name: &'static str) -> String {
    let mut f = File::open(name.to_string() + ".glsl").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    return s;
}
