use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

pub fn glob() {
    println!("----------- mem_stuff.rs -----------");
    ownership();
    borrowing();
    lifetime();

    ref_counted();
    atomic_counted();
    mutex();
}

fn mutex() {
    struct Person {
        name: Arc<String>,
        state: Arc<Mutex<String>>
    }

    impl Person {
        fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
            return Person { name: name, state: state};
        }

        fn greet(& self) {
            let mut state = self.state.lock().unwrap();
            state.clear();
            state.push_str("excited");
            println!("Hi my name is {}, and i am {}", self.name, state.as_str());
        }
    }

    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());

    t.join().unwrap();
}

fn atomic_counted() {
    struct Person {
        name: Arc<String>
    }

    impl Person {
        fn new(name: Arc<String>) -> Person {
            return Person { name: name};
        }

        fn greet(&self) {
            println!("Hi my name is {}", self.name)
        }
    }

    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {}", name);

    t.join().unwrap();
}

fn ref_counted() {

    struct Person {
        name: Rc<String>
    }

    impl Person {
        fn new(name: Rc<String>) -> Person {
            return Person { name: name};
        }

        fn greet(&self) {
            println!("Hi my name is {}", self.name)
        }
    }

    let name = Rc::new("John".to_string());
    let person = Person::new(name.clone());

    person.greet();
    // Does not work because we moved name to the person unless we use Rc
    println!("Name = {}", name);
}

fn lifetime() {
    // static is a lifetime of : lifes as long as the program.
    let s: &'static str = "Halu";

     struct Person {
        name: String
    }

    impl Person {
        // fn get_ref_name<'a>(&'a self) -> &'a String { <- What compiler does.
        fn get_ref_name(&self) -> &String {
            &self.name
        }
    }

    // This specifies a lifetime of z and tells the struct that the lifetime
    // of the Person is the same as the company.
    struct Company <'z>{
        name: String,
        ceo: &'z Person
    }

    let boss = Person {
        name: String::from("Elon Musk")
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss
    };

}

fn borrowing() {
    let print_vector = |x:&Vec<i32>|
        {
            println!("{:?}", x);
        };

    // By passing the address we can tell the func that it can borrow the value
    let v = vec![3,2,1];
    print_vector(&v);

    let mut a = 40;

    let b = &mut a;
    *b += 2;

    println!("a is {}", a);

    let mut z = vec![12,2];

    for i in &z {
        println!("i = {}", i);
        // Cant do this for somewhat obv reasons lol.
        // z.push(2);
    }
}

fn ownership() {
    // v owns memory on heap that represents the vec
    let v = vec![1,2,3];

    // this transfers ownership from v to v2. So v no longer owns the heap data
    let v2 = v;

    // This will not compile bcs it violates rust memory safety guaranteed
    // println!("{:?}", v);

    let foo = |v: Vec<i32>| ();

    foo(v2);
    // This too would no longer build because foo grabs ownership of v2
    // println!("{:?}", v2);

    // This works for primitive types tho
    let u = 1;
    let u2 = u;

    println!("u = {}", u);

    // If you have a pointer to a heap object the rules of ownership seem to apply.
    // Thus any variable that points to the heap will be moved on assignment.
    // But the behavior of primitives can be done by implemented a trait to copy
    // for heap memory.


    // You could do this hacky way where you would give back ownership
    // but its kinda whack.
    let print_vector = |x:Vec<i32>| -> Vec<i32>
        {
            println!("{:?}", x);
            return x;
        };

    let mut v3 = vec![3,2,1];
    v3 = print_vector(v3);
    println!("{:?}", v3);
}