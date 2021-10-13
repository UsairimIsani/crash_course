use std::{collections::HashMap, vec};

use crash_course::types;

fn main() {
    // u8, u16, u32, u64, u128, usize
    // i8, i16, i32, i64, i128, isize
    // bool
    // f32, f64

    // String
    // Vec
    // HashMap
    // BTreeMap
    //

    'loop1: loop {
        let mut y = 1;

        println!("Looping external");
        loop {
            y += 1;
            println!("Looping");
            if y == 5 {
                break 'loop1;
                // break;
            }
        }
        // break;
    }

    let a = [1, 2, 3];

    for i in 0..=5 {
        println!("Hello Infinity {}", i);
    }

    for i in a {
        println!("Hello Infinity {}", i);
    }

    for i in vec![1, 2, 3].iter_mut() {
        println!("Hello Infinity {}", i);
    }

    let a = vec![2, 23, 4];

    let c = a.iter().map(|f| (f, f * 2)).collect::<HashMap<_, _>>();

    let d = a.into_iter().map(|f| (f, f * 2)).collect::<HashMap<_, _>>();
    // a[23];

    let v = (1, 65u8, 4.9, "");

    // let a = 56 as f32;
    let a = 48 as f32;

    let b = hello();

    #[derive(PartialEq, PartialOrd)]
    enum Aplha {
        A,
        B,
    }

    let a = Aplha::A;

    if a == Aplha::A {}

    match a {
        Aplha::A => println!("Jljsl"),
        _ => (),
    }
}

fn hello() -> () {
    println!("Hello");
}

fn hello_world() -> (u32, usize) {
    (8, 9)
}

fn chicken_dinner(arm: String) -> String {
    return format!("{}", arm);
    format!("{}", arm)
}
