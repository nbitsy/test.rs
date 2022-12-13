
#[derive(Debug)]
enum IpAddr {
    V4,
    V6,
}

#[test]
fn test_enum_base() {
    let e = IpAddr::V4;
    let e6 = IpAddr::V6;
    println!("{:?} {:?}", e, e6);
    match e {
        IpAddr::V4 => {
            println!("I V4");
        }
        IpAddr::V6 => {
            println!("I V6");
        }
    }
}

#[derive(Debug)]
enum IPAddrDefault {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[test]
fn test_enum_value() {
    let e = IPAddrDefault::V4(127,0,0,1);
    let e6 = IPAddrDefault::V6(String::from(":::1"));
    println!("{:?}", e);
    match e {
        IPAddrDefault::V4(127,0,0,1) => {
            println!("{:?}", e);
        }
        _ => {
            println!("{:?}", e6);
        }
    }
}
