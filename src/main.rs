use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use term;

#[derive(Debug, Deserialize, Serialize)]
struct Spending {
    name: String,
    quantity: i64,
    price: f64
}

#[derive(Debug, Deserialize, Serialize)]
struct CreditCards {
    name: String,
    limit: f64,
    balance: f64
}

#[derive(Debug, Deserialize, Serialize)]
struct Budget {
    currency: String,
    spending: Vec<Spending>,
    monthly_income: f64,
    monthly_expenses: f64,
    credit_cards: Vec<CreditCards>
}

impl Budget {
    fn get_total(&self) -> f64 {
        let mut total: f64 = self.monthly_expenses;
        for expense in &self.spending {
            total += expense.price * expense.quantity as f64;
        }
        let credit_cards_total = self.get_total_credit_cards();

        total += credit_cards_total;
        
        total
    }

    fn get_total_credit_cards(&self) -> f64 {
        let mut total: f64 = 0.0;

        let mut terminal = term::stdout().unwrap();
        let credit_cards = &self.credit_cards;

        for credit_card in credit_cards {
            if credit_card.balance > credit_card.limit {
                terminal.fg(term::color::BRIGHT_RED).unwrap();
                print!("Credit card ");
                
                terminal.attr(term::Attr::Reverse).unwrap();
                terminal.attr(term::Attr::Bold).unwrap();
                
                print!("{}", credit_card.name);
                
                terminal.reset().unwrap();
                terminal.fg(term::color::BRIGHT_RED).unwrap();
                
                println!(" is over the limit");
                terminal.reset().unwrap();

                total += credit_card.balance;
                continue;
            }
    
            if credit_card.limit - credit_card.balance <= 100.0 && credit_card.limit - credit_card.balance > 80.0 {
                terminal.fg(term::color::BRIGHT_RED).unwrap();
    
                print!("Credit card ");
                terminal.attr(term::Attr::Reverse).unwrap();
                terminal.attr(term::Attr::Bold).unwrap();
                print!("{}", credit_card.name);
    
                terminal.reset().unwrap();
                terminal.fg(term::color::BRIGHT_RED).unwrap();
                println!(" is close to the limit or is at the limit");
    
                terminal.reset().unwrap();
                total += credit_card.balance;
                continue;
            }
    
            if credit_card.balance == 0.0 {
                println!("Credit card {} is not being used", credit_card.name);
                continue;
            }
    
            if credit_card.balance / credit_card.limit > 0.5 {
                terminal.fg(term::color::MAGENTA).unwrap();
                println!("Credit card {} is being used a lot", credit_card.name);
                terminal.reset().unwrap();
            }
    
            total += credit_card.balance;
        }
    
        return total;
    }
}

struct BudgetAnalysis<'a> {
    total: f64,
    budget_identified: &'a str,
    monthly_income: f64,
}

impl BudgetAnalysis<'_> {
    fn print_budget_analysis(&self) {
        let mut terminal = term::stdout().unwrap();
    
        println!("Budget identified: {}", self.budget_identified);
        println!("Monthly income: {}", self.monthly_income);
        terminal.fg(term::color::BRIGHT_YELLOW).unwrap();

        println!("============================================================");

        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();

        terminal.attr(term::Attr::Bold).unwrap();
        terminal.attr(term::Attr::Reverse).unwrap();
        terminal.attr(term::Attr::Blink).unwrap();
        print!("Total:");

        terminal.reset().unwrap();
        terminal.fg(term::color::BRIGHT_GREEN).unwrap();

        println!(" {}", self.total);
    }
}

fn main() {
    let file_content = read_file("src/budget.json");

    let budget: Budget = serde_json::from_str(&file_content).unwrap();
    
    let total = budget.get_total();

    let budget_identified = if total > budget.monthly_income { "You are over budget" } else { "You are under budget" };

    let budget_analysis = BudgetAnalysis { total, budget_identified, monthly_income: budget.monthly_income };
    
    budget_analysis.print_budget_analysis();
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