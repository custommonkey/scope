use glium::backend::glutin_backend::GlutinFacade;
use glium::Program;

use std::io::prelude::*;

use std::fs::File;

pub fn new(v_name: &'static str, f_name: &'static str, display: &GlutinFacade) -> Program {
    let vs = load_shader(v_name);
    let fs = load_shader(f_name);

    Program::from_source(display, &vs, &fs, None).unwrap()
}

fn load_shader(name: &'static str) -> String {
    let mut f = File::open(format!("shaders/{}.glsl", name)).unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();

    return s;
}
