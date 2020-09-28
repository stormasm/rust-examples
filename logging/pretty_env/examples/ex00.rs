extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();
    error!("error log");
    warn!("warn log");
    info!("info log");
    debug!("debug log");
    trace!("trace log");
}
