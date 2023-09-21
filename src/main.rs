use std::collections::HashMap;
use std::io;

struct Bill {
    name: String,
    amount: String,
}

impl Bill {
    fn new(name: String, amount: String) -> Self {
        Self { name, amount }
    }
}

fn get_user_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let input = buffer.trim().to_owned();
    if input.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Input is empty",
        ))
    } else {
        Ok(input)
    }
}

fn add_bill(bills: &mut HashMap<u8, Bill>, key: &mut u8) {
    println!("Type bill name");
    let name = get_user_input().unwrap_or("no name".to_string());
    println!("Type amount");
    let amount = get_user_input().unwrap_or("0".to_string());
    bills.insert(*key, Bill::new(name, amount));
    *key += 1;
}

fn get_bills(bills: &HashMap<u8, Bill>) {
    if bills.is_empty() {
        println!("no bills.")
    } else {
        for (key, val) in bills.iter() {
            println!("id: {} - name: {} - amount: {}", key, val.name, val.amount)
        }
    }
}

fn remove_bill(bills: &mut HashMap<u8, Bill>) {
    println!("Type bill ID");
    let input = get_user_input().expect("provide correct ID");
    let id = &input.parse().expect("this is not a number");
    bills.remove(id);
}

fn update_bill(bills: &mut HashMap<u8, Bill>) {
    println!("Type bill ID");
    let input = get_user_input().expect("provide correct ID");
    let id = &input.parse().expect("this is not a number");
    println!("Type updated amount");
    let amount = get_user_input().expect("provide correct amount");

    if let Some(bill) = bills.get_mut(id) {
        bill.amount = amount;
    }
}

fn go_back() {}

enum MenuOptions {
    AddBill,
    ViewBills,
    DeleteBill,
    UpdateBill,
}

impl MenuOptions {
    fn from_str(input: &str) -> Option<MenuOptions> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            "3" => Some(Self::DeleteBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }

    fn show() {
        println!(
            "\n == Manage Bills == \n 1. Add Bill \n 2. View Bills \n 3. Remove Bill \n 4. Update Bill \n"
        );
        println!("Enter selection:");
    }
}

fn main() {
    let mut bills: HashMap<u8, Bill> = HashMap::new();
    let mut current_key: u8 = 1;

    loop {
        MenuOptions::show();
        let input = get_user_input().expect("no data entered.");

        match MenuOptions::from_str(input.as_str()) {
            Some(MenuOptions::AddBill) => add_bill(&mut bills, &mut current_key),
            Some(MenuOptions::ViewBills) => get_bills(&bills),
            Some(MenuOptions::DeleteBill) => remove_bill(&mut bills),
            Some(MenuOptions::UpdateBill) => update_bill(&mut bills),
            None => println!("Pick from the available options."),
        }
    }
}
