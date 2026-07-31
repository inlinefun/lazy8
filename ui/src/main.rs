fn main() {
    let args = lazy8_common::parse_args();
    match lazy8_common::init_logger(args.debug_mode) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            panic!("Failed to init a logger")
        }
    }

    log::info!("INF!");
    log::warn!("WRN!");
    log::error!("ERR!");
    log::debug!("DBG!");
    log::trace!("TRC!");
}
