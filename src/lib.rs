use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    name: String,
    amount_paid: f64,
    net_balance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    users: Vec<User>,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Participant {
    name: String,
    weight: u8,
    fair_share: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    amount: f64,
    participants: Vec<Participant>,
}

impl Transaction {
    pub fn new(amount: f64, participants: Vec<Participant>) -> Self {
        Transaction {
            amount,
            participants,
        }
    }
}

impl Participant {
    pub fn new(name: &str, weight: u8) -> Self {
        Participant {
            name: name.to_string(),
            weight,
            fair_share: None,
        }
    }
}

impl Users {
    pub fn new() -> Self {
        Users {
            users: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn add_user(&mut self, name: String) {
        if self.users.iter().any(|u| u.name == name) {
            println!("User {} already exists.", name);
            return;
        }
        self.users.push(User {
            name,
            amount_paid: 0.0,
            net_balance: 0.0,
        });
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.iter().map(|u| u.name.clone()).collect()
    }

    pub fn remove_user(&mut self, name: String) {
        if !self.users.iter().any(|u| u.name == name) {
            println!("User {} does not exist.", name);
            return;
        }
        self.users.retain(|u| u.name != name);
    }

    pub fn record_payment(&mut self, user: &str, amount: f64) {
        match self.users.iter_mut().find(|u| u.name == user) {
            Some(u) => {
                u.amount_paid += amount;
                self.transactions.push(Transaction {
                    amount,
                    participants: self
                        .users
                        .iter()
                        .map(|u| Participant {
                            name: u.name.clone(),
                            weight: 1,
                            fair_share: Some(amount / self.users.len() as f64),
                        })
                        .collect(),
                });
            }
            None => {
                println!("User {} not found.", user);
            }
        }
    }

    pub fn record_weighted_payment(&mut self, user: &str, mut transaction: Transaction) {
        let amount = transaction.amount;
        let participants: Vec<String> = transaction
            .participants
            .iter()
            .map(|p| p.name.clone())
            .collect();
        // let all_users = self.users.keys().collect::<Vec<&String>>();
        let all_users: Vec<String> = self.users.iter().map(|u| u.name.clone()).collect();

        if !(participants.len() == all_users.len()
            && participants.iter().all(|p| all_users.contains(p)))
        {
            println!("Participants are invalid.");
            return;
        }

        calculate_fair_shares(&mut transaction);

        match self.users.iter_mut().find(|u| u.name == user) {
            Some(u) => {
                u.amount_paid += amount;
                println!("{} for user {} added.", amount, user);
                self.transactions.push(transaction);
            }
            None => {
                println!("User {} not found.", user);
            }
        }
    }

    pub fn remove_payment(&mut self, user: &str, amount: f64) {
        match self.users.iter_mut().find(|u| u.name == user) {
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

    pub fn calculate_total_payments(&mut self) -> Result<Vec<String>, String> {
        if self.users.len() < 2 {
            println!("Not enough users to calculate payments.");
            return Err("Not enough users".to_string());
        }

        let mut results = Vec::new();

        for user in &mut self.users {
            let mut total_fair_share = 0.0;
            for transaction in &self.transactions {
                total_fair_share += transaction
                    .participants
                    .iter()
                    .find(|p| p.name == user.name)
                    .unwrap()
                    .fair_share
                    .unwrap_or(0.0);
            }
            user.net_balance = user.amount_paid - total_fair_share;
        }

        // for transaction in &self.transactions {
        //     for participant in &transaction.participants {
        //         let participant_name = &participant.name;
        //         let fair_share = &participant.fair_share.unwrap_or(0.0);
        //         for (name, user) in &mut self.users {
        //             if name == participant_name {
        //                 user.net_balance += user.amount_paid - fair_share;
        //             }
        //         }
        //     }
        // }

        {
            let eps = 1e-6;
            let mut creditors: Vec<(String, f64)> = self
                .users
                .iter()
                .filter_map(|user| {
                    if user.net_balance > eps {
                        Some((user.name.clone(), user.net_balance))
                    } else {
                        None
                    }
                })
                .collect();

            let mut debtors: Vec<(String, f64)> = self
                .users
                .iter()
                .filter_map(|user| {
                    if user.net_balance < -eps {
                        Some((user.name.clone(), user.net_balance))
                    } else {
                        None
                    }
                })
                .collect();

            creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            debtors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let mut i = 0usize;
            let mut j = 0usize;
            while i < creditors.len() && j < debtors.len() {
                let available = creditors[i].1;
                let owed = -debtors[j].1;
                let transfer = available.min(owed);

                results.push(format!(
                    "{} should pay {} {:.2}",
                    debtors[j].0, creditors[i].0, transfer
                ));

                creditors[i].1 -= transfer;
                debtors[j].1 += transfer; // debtors store negative values

                if creditors[i].1 <= eps {
                    i += 1;
                }
                if debtors[j].1 >= -eps {
                    j += 1;
                }
            }

            for user in &mut self.users {
                if user.net_balance.abs() <= eps {
                    user.net_balance = 0.0;
                }
            }
            Ok(results)
        }
    }

    pub fn settle_up(&mut self) {
        for user in &mut self.users {
            user.net_balance = 0.0;
            user.amount_paid = 0.0;
        }
        self.transactions.clear();
    }

    pub fn save_to_file(&self, file_path: &str) {
        match serde_json::to_writer_pretty(std::fs::File::create(file_path).unwrap(), &self) {
            Ok(_) => {}
            Err(e) => println!("Error saving users: {}", e),
        }
    }
}

pub fn load_from_file(file_path: &str) -> Option<Users> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let users_str: Users = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            return None;
        }
    };
    Some(users_str)
}

fn calculate_fair_shares(transaction: &mut Transaction) {
    let amount = transaction.amount;
    let total_weight: u32 = transaction
        .participants
        .iter()
        .map(|p| p.weight)
        .sum::<u8>() as u32;
    for p in &mut transaction.participants {
        p.fair_share = Some(amount * (p.weight as f64) / (total_weight as f64));
    }
}

pub mod tui;
