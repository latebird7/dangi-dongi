use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug)]
struct User {
    name: String,
    amount_paid: f64,
    net_balance: f64,
}

#[derive(Debug)]
struct Users {
    users: HashMap<String, User>,
}

impl Users {
    fn add_user(&mut self, name: String) {
        if self.users.contains_key(&name) {
            println!("User {} already exists.", name);
            return;
        }
        self.users.insert(
            name.clone(),
            User {
                name: name.clone(),
                amount_paid: 0.0,
                net_balance: 0.0,
            },
        );
        println!("User {} added.", name);
    }

    fn remove_user(&mut self, name: String) {
        if !self.users.contains_key(&name) {
            println!("User {} does not exist.", name);
            return;
        }
        self.users.remove(&name);
        println!("User {} removed.", name);
    }

    fn record_payment(&mut self, user: &str, amount: f64) {
        match self.users.get_mut(user) {
            Some(u) => {
                u.amount_paid += amount;
                println!("{} for user {} added.", amount, user);
            }
            None => {
                println!("User {} not found.", user);
            }
        }
    }

    fn remove_payment(&mut self, user: &str, amount: f64) {
        match self.users.get_mut(user) {
            Some(u) => {
                u.amount_paid -= amount;
                if u.amount_paid < 0.0 {
                    println!("Amount paid for user {} is negative. Setting to 0.", user);
                    u.amount_paid = 0.0;
                }
                println!("{} for user {} removed.", amount, user);
            }
            None => {
                println!("User {} not found.", user);
            }
        }
    }

    fn calculate_total_payments(&mut self) -> () {
        if self.users.len() < 2 {
            println!("Not enough users to calculate payments.");
            return;
        }
        let total: f64 = self.users.values().map(|u| u.amount_paid).sum();
        dbg!(total);
        let share_per_user = total / self.users.len() as f64;

        for (_, value) in &mut self.users {
            value.net_balance = value.amount_paid - share_per_user;
        }
    }
}

fn main() {
    let mut users = Users {
        users: HashMap::new(),
    };
    users.add_user(String::from("A"));
    users.add_user(String::from("B"));
    users.add_user(String::from("C"));
    users.record_payment("A", 60.0);
    users.record_payment("B", 30.0);
    users.record_payment("C", 30.0);

    users.remove_payment("B", 60.0);
    users.remove_payment("D", 20.0);
    
    users.calculate_total_payments();
    println!("users {:#?}", users.users);

    users.remove_user(String::from("B"));
    users.record_payment("B", 30.0);
    users.calculate_total_payments();

    println!("users {:#?}", users.users);
}
