use log::*;
use pretty_env_logger;

fn main() {
    pretty_env_logger::init();
    error!("error log");
    warn!("warn log");
    info!("info log");
    debug!("debug log");
    trace!("trace log");
}
