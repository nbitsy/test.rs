mod tests;

extern crate core;

use bwserver::tool::l4rs;

fn main() {
    let args = std::env::args();
    for (i, x) in args.enumerate() {
        println!("[{}] {}", i, x);
    }
}