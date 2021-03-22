#![allow(dead_code)]

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    return Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {
    println!("----------- sh.rs -----------");

    let x = Box::new(5);

    let p1 = origin();
    let p2 = Box::new(origin());

    let p2_val = *p2;

    // WILL NOT WORK BECAUSE VALUE IS MOVED OR SMTH WHO TF KNOWS
    // let p2_x = p2.x; // implicitly unboxing
}