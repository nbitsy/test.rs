use std::backtrace::Backtrace;
use std::collections::{HashMap, LinkedList};
use std::env::Args;
use std::os::unix::thread::JoinHandleExt;
use std::{panic, thread};
use std::panic::{catch_unwind, RefUnwindSafe, set_hook, UnwindSafe};
use std::sync::Arc;
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;

#[test]
fn test_panic_1() {
    panic!("ho no!");
}

fn zero() -> i32 {
    0
}

#[test]
fn test_panic_unwind() {
    let r = catch_unwind(|| {
        println!("hello i'm ok!");
    });
    println!("{:?}", r);

    let r = catch_unwind(|| {
        let v = 100;
        let v = v / zero();
    });
    println!("After panic: {:?}", r);
    println!("Here is OK!");
}

fn catch_thread() {
    use bwserver::util::thread::runner::Runner;

    let mut runner = Runner::run(|| {
        let v = 100;
        loop {
            eprintln!("Before Dive Zero==================");
            let v = v / zero();
            sleep(Duration::from_secs(10));
            println!("After panic!!!!!!!!!!!!!!!!!!!!!");
        }
    });

    runner.join().unwrap();
    println!("Run here!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
}

fn catch_curr_thread() {
    let r = catch_unwind(|| {
        let v = 100;
        let v = v / zero();
    });

    println!("Run here!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
}

fn no_catch() {
    let v = 100;
    let v = v / zero();
    println!("Can Not Run here!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
}

#[test]
fn test_panic_unwind_1() {
    bwserver::tool::l4rs::init_file("./config/log4rs.yaml").unwrap();
    catch_thread();
    //catch_curr_thread();
    //no_catch();
}