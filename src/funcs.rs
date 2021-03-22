use std::fmt::Debug;

struct Point {
    x: i64
}

impl Point{
    fn pow(&self) -> i64 {
        let p = &self.x * &self.x;
        return p;
    }
}

// this is a normal function
pub fn glob() {
    println!("----------- funcs.rs -----------");

    let p = Point{x: 2};
    println!("pow of point {}", p.pow());

    closures();

    traits();

    into_();

    drop_();
}

struct Enemy {
    name: String
}

impl Enemy {
    fn new<S: Into<String>>(name: S) -> Enemy {
        let n = name.into();
        println!("{} enters the game", n);
        return Enemy{name: n};
    }
}

impl Drop for Enemy {
    fn drop(&mut self) {
        println!("enemy {} has died", self.name);
    }
}

fn drop_() {
    let goblin = Enemy::new("Jeff");
    println!("Somethign happens");
    // drop(goblin); Just moves the value and thus its dead
}

struct Person {
    name: String
}

impl Person {

    // Will not work for proper strings would always need conversions etc
    // fn new(name: &str) -> Person {
    //     return Person{name: name.to_string()s};
    // }

    fn new<S: Into<String>>(name: S) -> Person {
        return Person {name: name.into()};
    }
}

fn into_() {
    let p = Person::new("Test");

    let name = String::from("Test2");

    let p2 = Person::new(name);
}

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name())
    }
}

#[derive(Debug)]
struct Human {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        return Human{name:name};
    }

    fn name(&self) -> &'static str {
        return self.name;
    }

    fn talk(&self) {
        println!("Hi my name is {}", self.name);
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {
            result += *x;
        }
        return result;
    }
}

fn traits() {
    let h = Human{name: "John"};

    let h2 = Human::create("John2");
    let _h3: Human = Animal::create("John2");

    h.talk();
    h2.talk();

    let v = vec![3,2,1];

    println!("i can now sum a vec: {}", v.sum());

    print_animal(h);
}

// fn print_animal(animal: impl Animal + Debug) {
// fn print_animal<T: Animal + Debug>(animal: T) {
fn print_animal<T>(animal: T) where T: Animal + Debug {
    println!("{:?}", animal);
    println!("The animal's name is {}", animal.name());
}

fn is_even(x: u32) -> bool{
    x % 2 == 0
}

fn closures() {
    let plus_one = |x:i32| -> i32 { x+1 };

    let a = 3;
    println!("+1 e: {}", plus_one(a));

    let plus_two = |x| { x+2.0 };

    println!("+2 : {}", plus_two(4.0));

    // T by value
    // T&
    // &mut &
    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f: {}", f);

    let sum = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < 500)
        .filter(|x: &u32| is_even(*x))
        .fold(0, |sum, x| sum + x);

    println!("sum haskell weirdness {}", sum);
}

