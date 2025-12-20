#[derive(Clone, Debug)]
struct User {
    name: String,
    amount_paid: f64,
    net_balance: f64,
}

#[derive(Debug)]
struct Users {
    users: Vec<User>,
}

impl Users {
    fn add_user(&mut self, name: String) {
        // Check if user already exists:
        for user in &self.users {
            if user.name == name {
                println!("User {} already exists.", name);
                return;
            }
        }
        self.users.push(User {
            name: name.clone(),
            amount_paid: 0.0,
            net_balance: 0.0,
        });
        println!("User {} added.", name);
    }

    fn record_payment(&mut self, user: &str, amount: f64) {
        let mut found = false;
        for u in self.users.iter_mut() {
            if u.name == user {
                u.amount_paid += amount;
                found = true;
                println!("User {} paid {}.", user, amount);
            }
        }
        if !found {
            println!("User {} not found.", user);
        }
    }

    fn remove_payment(&mut self, user: &str, amount: f64) {
        let mut found = false;
        for u in self.users.iter_mut() {
            if u.name == user {
                u.amount_paid -= amount;
                found = true;
                println!("{} for user {} removed.", amount, user);
            }
        }
        if !found {
            println!("User {} not found.", user);
        }
    }

    fn calculate_total_payments(&mut self) -> () {
        if self.users.len() == 0 {
            println!("No users to calculate payments for.");
            return;
        }
        let total: f64 = self.users.iter().map(|u| u.amount_paid).sum();
        let share_per_person = total / self.users.len() as f64;

        for user in &mut self.users {
            user.net_balance = user.amount_paid - share_per_person;
        }

        println!(
            "self.users after calculating total payments: {:#?}",
            self.users
        );
    }
}

fn main() {
    let mut users = Users { users: Vec::new() };
    users.add_user(String::from("A"));
    users.add_user(String::from("B"));
    users.add_user(String::from("C"));
    users.record_payment("A", 50.0);
    users.record_payment("B", 30.0);
    users.calculate_total_payments();

    println!("users {:#?}", users.users);

}
