
#[cfg(config = "debug")]
fn compile_when_debug() {
    println!("debug")
}

#[cfg(config = "release")]
fn compile_when_release() {
    println!("debug")
}

fn test_cfg() {
    if cfg!(config = "debug") {
        // compile_when_debug();
    } else if cfg!(config = "release") {
        // compile_when_release();
    }
}

fn main() {
    test_cfg()
}
