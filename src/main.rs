use dangi_dongi::{Participant, Transaction, Users};

fn main() {
    let mut users = Users::new();
    users.add_user(String::from("A"));
    users.add_user(String::from("B"));
    users.add_user(String::from("C"));
    users.record_payment("A", 60.0);
    users.record_payment("B", 30.0);
    let transaction1 = Transaction::new(
        30.0,
        vec![
            Participant::new("A", 2),
            Participant::new("B", 1),
            Participant::new("C", 1),
        ],
    );
    users.record_weighted_payment("C", transaction1);
    users.calculate_total_payments();

    users.save_to_file("./db.json");
}
