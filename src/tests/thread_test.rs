use std::rc::Rc;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::spawn;
use std::time::Duration;

#[test]
fn test_thread_base() {
    let (sx,rx) = channel();
    let sx2 = Sender::clone(&sx);

    let t = spawn(move || {
        for i in 1..10 {
            sx.send("Hello From Sub1");
        }
    });


    let t2 = spawn(move || {
        for i in 1..10 {
            sx2.send("Hello From Sub2");
        }
    });

    for r in rx {
        println!("Recv from sub: {:?}", r);
    }

    t.join().expect("TODO: panic message");
    t2.join().expect("TODO: panic message");
}

#[test]
fn test_mutex() {
    let m = Mutex::new(100);
    println!("{:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num *= 6;
    }

    println!("{:?}", m);
}

// XXX: 只有通过 Arc 来实现对同一块区域的引用才能实现内存共享
#[test]
fn test_mutex_1() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let c = Arc::clone(&m);
        let h = spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join();
    }

    println!("{:?}", m);
}
