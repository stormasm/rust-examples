extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();
    info!("such information");
    warn!("o_O");
    error!("much error");
}
