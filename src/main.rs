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

        {
            let eps = 1e-6;
            let mut creditors: Vec<(String, f64)> = self
                .users
                .iter()
                .filter_map(|(name, u)| if u.net_balance > eps { Some((name.clone(), u.net_balance)) } else { None })
                .collect();

            let mut debtors: Vec<(String, f64)> = self
                .users
                .iter()
                .filter_map(|(name, u)| if u.net_balance < -eps { Some((name.clone(), u.net_balance)) } else { None })
                .collect();

            creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            debtors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let mut i = 0usize;
            let mut j = 0usize;
            while i < creditors.len() && j < debtors.len() {
                let available = creditors[i].1;
                let owed = -debtors[j].1;
                let transfer = available.min(owed);

                println!("{} pays {} {:.2}", debtors[j].0, creditors[i].0, transfer);

                creditors[i].1 -= transfer;
                debtors[j].1 += transfer; // debtors store negative values

                if creditors[i].1 <= eps { i += 1; }
                if debtors[j].1 >= -eps { j += 1; }
            }

            for (_, u) in &mut self.users {
                if u.net_balance.abs() <= eps {
                    u.net_balance = 0.0;
                }
            }
        }
    }

    fn settle_up(&mut self) {
        for (_, u) in &mut self.users {
            u.net_balance = 0.0;
            u.amount_paid = 0.0;
        }
        println!("All users have been settled up!");
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
    users.record_payment("C", 40.0);
    
    users.calculate_total_payments();

    users.settle_up();
    println!("{:#?}", users.users);

}
