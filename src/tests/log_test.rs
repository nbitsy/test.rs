use bwserver::tool::l4rs;
use log::{error, info};

#[test]
fn test_log() {
    let logfile = "config/log4rs.yaml";
    if l4rs::init_file(logfile) == None {
        panic!("Init log file {:?} error.", logfile);
    }

    info!("hello");
    error!("world");
}

#[test]
fn test_log_default() {
    if l4rs::init_default() == None {
        panic!("");
    }

    info!("default hello");
    error!("default world");
}

#[test]
fn test_log_file_or() {
    if l4rs::init_file_or("./orlog.log") == None {
        panic!("");
    }

    info!("or hello");
    error!("or world");
}