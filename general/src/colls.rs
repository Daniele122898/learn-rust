use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

pub fn glob() {
    println!("----------- colls.rs -----------");
    vectors();

    hashmaps();

    hashset();

    iterators();
}

fn iterators() {
    let vec = vec![3,2,1];

    // To mutate elements make vec mut and use iter_mut!
    for x in vec.iter() {
        println!("{}", x);
    }

    for x in vec.iter().rev() {
        println!("rev {}", x);
    }

    let mut vec2= vec![1,2,3];
    // vec.into_iter, moves the entire vector
    vec2.extend(vec);
    print!("{:?}", vec2);
}

fn hashset() {
    let mut greeks = HashSet::new();
    greeks.insert("delta");
    greeks.insert("gamma");
    greeks.insert("delta");

    println!("{:?}", greeks);

    if greeks.insert("vega") {
        println!("added vega");
    }

    if !greeks.contains("kappa") {
        println!("we dont have a kappa");
    }

    // Create new sets
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // check subsets
    println!("is {:?} a subset of {:?}? {}",
             _2_8, _1_10, _2_8.is_subset(&_1_10));

    // disjoint?
    println!("is {:?} disjoint from {:?}? {}",
             _1_5, _6_10, _1_5.is_subset(&_6_10));

    //etc

}

fn hashmaps() {
    let mut shape = HashMap::new();

    shape.insert(String::from("triangle"), 3);
    shape.insert(String::from("square"), 2);

    println!("a square has {} sides", shape["triangle"]);
    // This will panic !
    // println!("a square has {} sides", shape["tr"]);
    // use this to check
    match shape.get("tr") {
        None => {
            println!("tr is not in hasmap");
        }
        Some(sides) => {
            println!("it has {} sides", sides);
        }
    }

    // Insert if it doesnt exist already
    shape.entry("circle".into()).or_insert(1);
}

fn vectors() {
    let mut a = Vec::new();
    for i in 0..1000000 {
        a.push(i);
    }

    // println!("{:?}", a);

    // Returns option type
    match a.get(10) {
        Some(x) => println!("Got vec element at pos 10"),
        None => println!("None :D")
    }

    if let Some(z) = a.get(20) {
        println!("succeeded {}", z);
    }


    let mut sum: u64 = 0;

    let now = std::time::SystemTime::now();
    for x in &a {
        sum += x;
    }
    println!("& needed: {}", now.elapsed().ok().unwrap().as_millis());

    sum = 0;
    let now = std::time::SystemTime::now();
    for &x in a.iter() {
        sum += x;
    }
    println!("iter needed: {}", now.elapsed().ok().unwrap().as_millis());

    sum = 0;
    let now = std::time::SystemTime::now();
    for x in a {
        sum += x;
    }
    println!("non& needed: {}", now.elapsed().ok().unwrap().as_millis());

}