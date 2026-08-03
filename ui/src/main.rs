use std::fs;

use lazy8_core::CHIP8;

fn main() {
    let args = lazy8_common::parse_args();
    match lazy8_common::init_logger(args.debug_mode) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            panic!("Failed to init a logger")
        }
    }

    let mut chip8 = CHIP8::default();

    match args.rom_file {
        Some(path) => {
            log::trace!("recieved rom file: {}", &path);
            match fs::read(&path) {
                Ok(data) => chip8.load(data),
                Err(e) => {
                    log::error!("{}", e);
                    panic!("Failed to load rom file {}", &path)
                }
            }
        }
        None => (),
    }

    loop {
        chip8.step();
    }
}
