use std::ops::Deref;

struct DerefValue<T> {
    value: T,
}

impl<T> DerefValue<T> {
    pub fn new(v: T) -> Self {
        return Self { value: v };
    }
}

impl<T> Deref for DerefValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

fn deref_value(v : &i32) {
    println!("v: {:?}", v);
}

#[test]
fn test_deref_base() {
    let v = DerefValue::new(100);
    let vv : i32 = *v.deref();
    deref_value(&v);
}
