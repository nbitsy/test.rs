use std::backtrace::Backtrace;
use std::collections::{HashMap, LinkedList};
use std::env::Args;
use std::os::unix::thread::JoinHandleExt;
use std::{panic, thread};
use std::panic::{catch_unwind, RefUnwindSafe, set_hook, UnwindSafe};
use std::sync::Arc;
use std::thread::{JoinHandle, spawn};

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

#[derive(Debug)]
struct ThreadRunner<F> {
    func: F,
}

impl<F> ThreadRunner<F>
    where F: FnOnce() + UnwindSafe + Copy + Send + Sync + 'static {
    fn run(f: F) -> ThreadRunner<F> {
        /// simple log
        set_hook(Box::new(|panic_info| {
            if let Some(location) = panic_info.location() {
                if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
                    println!("[PANIC] {:?}:{:?}", location.to_string(), payload);
                } else {
                    println!("[PANIC] {:?}", location.to_string());
                }
            } else {
                println!("[PANIC]!!!!!!!!!!!!!")
            }
        }));

        let handle = spawn(move || {
            let r = catch_unwind(f);
        });

        handle.join().unwrap();
        let runner = ThreadRunner { func: f };
        return runner;
    }

    fn join(&self) {
    }
}

fn catch_thread() {
    let ctx = ThreadRunner::run(|| {
        let v = 100;
        eprintln!("Before Dive Zero==================");
        let v = v / zero();
    });

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
    catch_thread();
    //catch_curr_thread();
    //no_catch();
}