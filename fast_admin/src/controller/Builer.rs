struct User {
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

struct UserBuilder {
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl UserBuilder {
    fn new(id: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            email: email.into(),
            first_name: None,
            last_name: None,
        }
    }
    fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    fn last_name(mut self, last_name: impl Into<String>) -> UserBuilder {
        self.first_name = Some(last_name.into());
        self
    }
}


use std::sync::{Mutex, TryLockResult};

#[test]
fn test() {
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock(); // try to get the lock

    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {}", value)
    } else {
        println!("Didn't get the lock")
    }

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock();

    match other_mutex_changer {
        Ok(_) => {}
        Err(_) => {}
    }
}

