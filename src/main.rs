extern crate core;

use log;
use bwserver;

fn main() {
    let args = std::env::args();
    for (i, x) in args.enumerate() {
        println!("[{}] {}", i, x);
    }

    let logfile = "config/log4rs.yaml";
    if let Some(_r) = bwserver::util::l4rs::init_file(logfile) {
        // do nothing
    } else {
        panic!("Init log file {:?} error.", logfile);
    }

    log::info!("hello");
    log::error!("world");
}