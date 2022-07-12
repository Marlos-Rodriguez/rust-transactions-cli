pub mod clients;
pub mod transactions;

use std::env;
use std::fs;

use clients::models::Client;
use transactions::models::Transaction;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &String = match args.get(1) {
        Some(x) => x,
        None => {
            println!("Path for CSV file is needed");
            return;
        }
    };

    let tx: String = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(e) => {
            println!("Something went wrong reading the file {}", e);
            return;
        }
    };

    let transactions: Vec<Transaction> = Transaction::get_transactions(tx);

    let clients: Vec<Client> = Client::process_transactions(&transactions);

    let data: String = Client::clients_to_csv(clients);
    println!("{}", data);
}
