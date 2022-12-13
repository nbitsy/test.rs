#[test]
fn test_if() {
    let x = 5;
    if x == 5 {
        println!("Hello");
    } else {
        println!("World");
    }

    let y = if x == 5 {
        "Hello"
    } else {
        "World"
    };

    println!("{}", y);
}

#[test]
fn test_while() {
    let mut x = 100;
    while x >= 0 {
        println!("{}", x);
        x = x - 1;
    }

    x = 9;
    let y = loop {
        if x >= 10 {
            break x;
        }
        break 8;
    };

    println!("{}", y)
}

#[test]
fn test_for() {
    let v = [1, 2, 3, 4];
    for i in v {
        println!("{}", i)
    }
}

#[test]
fn test_iter() {
    let v = [1, 2, 3, 4, ];

    for x in v.iter() {
        println!("{}", x)
    }

    for x in v.iter().rev() {
        println!("{}", x);
    }
}