use std::mem;

pub fn glob() {
    println!("----------- types.rs -----------");

    option_type();

    arrays();

    slices();

    tuples();

    generics();

    strings();
}

fn strings() {
    let s = "hello there"; // &str = string slice static allocated.

    for c in s.chars() {
        println!("{}", c);
    }

    // String heap allocated
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // concat
    // let _z = letters + &letters;

    let name = "boi";
    let greeting = format!("hi, im {}", name);
    println!("{}", greeting);
}

fn generics() {
    struct Point<T, V> {
        x: T,
        y: V
    }

    struct Line<T> {
        start: Point<T,T>,
        end: Point<T,T>
    }

    let a: Point<u16, u32> = Point {x: 1, y: 2};
    let b = Point {x: 1.2, y: 2.2};
}

fn tuples() {
    let tpl = (1,2);
    
    // Destructuring
    let (a,b) = tpl;
}

fn use_slice(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    println!("{:?}", slice);
    slice[0] = -1;
}

fn slices() {
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]); // pass slice
    use_slice(&mut data); // pass all of array
}

fn arrays() {
    let a:[i32;5] = [1,2,3,4,5];

    println!("a has {} elemens and first is {}", a.len(), a[0]);

    println!("{:?}", a);

    let b = [1u16;10]; // 10 elements of value 1 with type u16

    for i in 0..b.len() {
        println!("{}", b[i])
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0]
    ];
    println!("{:?}", mtx);
}

fn option_type() {
    let x = 3;
    let y = 0;

    // Option -> Some(v)  or None
    let result = if y != 0 {
            Some(x/y)
        } else {
            None
        };

    match result {
        Some(r) => {
            println!("succeeded some {}", r);
        }
        None => {
            println!("failed option");
        }
    }

    // Or use if

    if let Some(z) = result {
        println!("succeeded {}", z);
    }
}