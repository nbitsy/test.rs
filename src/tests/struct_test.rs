#[derive(Debug)]
struct Rect {
    pub(super) width: i32,
    height: i32,
}

#[test]
fn test_struct_base() {
    let r = Rect { width: 90, height: 100 };
    println!("{:?}", r);
}

impl Rect {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
}

impl Rect {
    fn set_width(&mut self, v: i32) {
        self.width = v;
    }

    fn set_height(&mut self, v: i32) {
        self.height = v
    }
}

#[test]
fn test_struct_impl() {
    let mut r = Rect { width: 8, height: 9 };
    println!("{}", r.area());
    r.set_width(10);
    r.set_height(100);
    r.width = 11;
    println!("{}", r.area());
}

#[derive(Debug)]
struct Point(i32, i32, i32);

#[test]
fn test_struct_tup() {
    let p = Point(9, 90, 990);
    println!("{:?}", p);
    println!("{}", p.0);
    println!("{}", p.1);
    println!("{}", p.2);
}