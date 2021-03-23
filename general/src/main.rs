mod sh;
mod comb_lock;
mod types;
mod colls;
mod funcs;
mod mem_stuff;
mod circ_ref;
mod threads;

    use std::mem;

    // no memory address bcs at compile time it just gets copy pasted where its needed
    const MEANING_OF_LIFE: u8 = 42;

    // global but with memory address
    static zz: u8 = 43;

    fn main() {
        print_shit();

        scope();

        sh::stack_and_heap();

        loops();

        matching();

        // comb_lock::enter_lock();

        types::glob();
        colls::glob();
        funcs::glob();
        mem_stuff::glob();
        circ_ref::glob();
        threads::glob();
    }


    fn matching() {
        let country_code = 41;

        let country = match country_code {
        44 => "UK",
        41 => "Your mom",
        12| 2 => "us or maybe brazil you'll never know kek",
        _ if (country_code % 2 == 0) => "Even country",
        _r @ 1..=1000 => {
            println!("cc {}", country_code);
            "Unknown"
        }, // r is now the range of 1-1000
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country_code);
}

fn loops() {
    for x in 1..11 {
        //
    }

    while 0 > 1 {
        //
    }

    loop { // while true
        break;
    }

    for (pos, val) in (30..41).enumerate() {
        //
    }
}

fn scope() {
    // there are C syle scopes
    let a = 0;
    {
        let a = 2;
        println!("a inner {}", a)
    }
    println!("a outer {}", a)
}

fn print_shit() {
    println!("Hello world");
    // let is immutable.
    let a: u8 = 123;
    println!("a = {}", a);

    let mut b: i8 = 0;
    // we can change the value now :D
    b = 10;

    // can do similar type inference like go
    let c = 1231213;
    println!("c = {}, nr of btyes = {}", c, mem::size_of_val(&c));

    // usize and isize are sized to the operating system. so for us it would be 64 bit
    let z: isize = 123;
    println!("Z has size = {}", mem::size_of_val(&z));

    // in rust a float is 64 bit on my machine.
    let f = 2.5;

    let cubed = i32::pow(10,3);
    // This pow f uses taylor series. use powi for normal powers.
    let fcube = f64::powf(3.2, 3.1);

    let pi = std::f64::consts::PI;
}
