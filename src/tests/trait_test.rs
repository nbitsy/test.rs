use std::any::{Any, type_name};

#[derive(Debug)]
struct MyCell<T: ?Sized> {
    value: T
}

#[test]
fn test_trait() {
    let c = MyCell{value: 100};
    println!("{}", type_name::<i32>());
    println!("{:?}", c.type_id());
    //println!("{:?}, {:?}", c, type_name_of_val(&c));
}

#[derive(Debug)]
struct YaCell<T> {
    value: T
}

#[test]
fn test_trait_1() {
    let c = MyCell{value: 90};
    let c1 = MyCell{value: YaCell{value: 99}, };
    println!("{:?}", c1);
    let c2 = MyCell{value: YaCell{value: &[1,2,3]}, };
    println!("{:?}", c2);

    let v = &vec![1,2,3];
    let c3 = MyCell{value: YaCell{value: v, }};
    println!("{:?}", c3);

    let vv = Vec::from([1,2,3]);
    let vvv = vv.as_slice();
    println!("{:?}", vvv);
    let c4 = MyCell{value: YaCell{value: vvv, }};
    println!("{:?}", c4);
}


trait IObject {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn typ(&self) -> i32;
    fn to_string(&self) -> String;

    fn set_id(&mut self, id: u64);
    fn set_name(&mut self, name: &str);
    fn set_typ(&mut self, typ: i32);
}

#[derive(Debug)]
struct Object {
    id: u64,
    name: String,
    typ: i32,
}

#[derive(Debug)]
struct Creature {
    object: Object,
    hp: i32,
    mp: i32,
    atk: i32,
}

impl Object {
    pub fn new() -> Object {
        return Object {
            id: 0,
            name: "unknown".to_string(),
            typ: 0,
        };
    }
}

impl IObject for Object {
    fn id(&self) -> u64 {
        return self.id;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn typ(&self) -> i32 {
        return self.typ;
    }

    fn to_string(&self) -> String {
        return "todo".to_string();
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn set_typ(&mut self, typ: i32) {
        self.typ = typ;
    }
}

impl Creature {
    pub fn new() -> Creature {
        return Creature {
            object: Object::new(),
            hp: 0,
            mp: 0,
            atk: 0,
        };
    }
}

impl IObject for Creature {
    fn id(&self) -> u64 {
        return self.object.id;
    }

    fn name(&self) -> &str {
        return &self.object.name;
    }

    fn typ(&self) -> i32 {
        return self.object.typ;
    }

    fn to_string(&self) -> String {
        return "todo".to_string();
    }

    fn set_id(&mut self, id: u64) {
        self.object.set_id(id);
    }

    fn set_name(&mut self, name: &str) {
        self.object.set_name(name);
    }

    fn set_typ(&mut self, typ: i32) {
        self.object.set_typ(typ);
    }
}

fn pass_iobject(o: &mut impl IObject) -> bool {
    o.set_id(1);
    o.set_name("oooooo");
    o.set_typ(1111);
    return false;
}

#[test]
fn test_trait_base() {
    let mut o = Object::new();
    let mut c = Creature::new();

    pass_iobject(&mut o);
    pass_iobject(&mut c);

    println!("{:?}", o);
    println!("{:?}", c);
}
