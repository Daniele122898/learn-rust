use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

pub fn enter_lock() {
    println!("----------- comp_lock.rs -----------");

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        // get rid of linebreak
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}