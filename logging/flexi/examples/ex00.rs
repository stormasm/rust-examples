use flexi_logger::Logger;
use log::*;

fn main() {
    Logger::with_env().start().unwrap();
    error!("error log");
    warn!("warn log");
    info!("info log");
    debug!("debug log");
    trace!("trace log");
}
