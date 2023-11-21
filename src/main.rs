use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Shopping {
    name: String,
    quantity: i64,
    price: f64
}

#[derive(Debug, Deserialize, Serialize)]
struct Budget {
    budget: i64,
    currency: String,
    shopping: Vec<Shopping>,
}

fn main() {
    let file_content = read_file("src/budget.json");
    
    let budget: Budget = serde_json::from_str(&file_content).unwrap();
    
    let mut total: f64 = 0.0;

    for shopping in budget.shopping {
        total += shopping.price * shopping.quantity as f64;
        println!("Shopping: {:?}", shopping);

        identify_budget(total);
    }
}

fn identify_budget(total: f64) {
    if total > 100.0 {
        println!("You are over budget");
    } else {
        println!("You are under budget");
    }
}

fn read_file(file_name: &str) -> String {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Something went wrong reading the file");

    return contents;
}