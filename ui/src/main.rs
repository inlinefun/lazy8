mod window;

use std::fs;

use winit::event_loop::{ControlFlow, EventLoop};

use crate::window::App;

fn main() {
    let args = lazy8_common::parse_args();
    match lazy8_common::init_logger(args.debug_mode) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            panic!("Failed to init a logger");
        }
    }

    let mut app = App::default();

    let path = match args.rom_file {
        Some(it) => it,
        None => "default.ch8".to_string(),
    };
    log::debug!("recieved rom file: {}", &path);
    match fs::read(&path) {
        Ok(data) => app.chip8.load(data),
        Err(e) => {
            log::error!("{}", e);
            log::error!("Failed to load rom file {}", &path);
            return;
        }
    }

    let event_loop = match EventLoop::new() {
        Ok(it) => it,
        Err(e) => {
            log::debug!("{}", e);
            log::error!("Failed to init an event loop for winit");
            return;
        }
    };
    event_loop.set_control_flow(ControlFlow::Poll);

    match event_loop.run_app(&mut app) {
        Ok(_) => (),
        Err(e) => {
            log::debug!("{}", e);
            log::error!("Failed to run winit app");
            return;
        }
    }
}
