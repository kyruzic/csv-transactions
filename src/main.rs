mod clients;
mod fixed_number;
mod transactions;

extern crate core;

use crate::clients::create_client;
use crate::transactions::{format_transaction, InputTransaction};
use clients::Client;
use csv::{Reader, ReaderBuilder, Trim};
use std::env;
use std::fs::File;

fn main() {
    let rdr = get_rdr();
    let mut clients: Vec<Client> = Vec::new();

    rdr.into_deserialize::<InputTransaction>()
        .filter_map(|input_transaction| input_transaction.ok())
        .map(format_transaction)
        .for_each(|tx| match tx {
            Ok(tx) => {
                let client = clients.iter_mut().find(|client| client.id == tx.client_id);
                match client {
                    None => {
                        let mut client = create_client(tx.client_id);
                        tx.calculate_transaction(&mut client);
                        clients.push(client);
                    }
                    Some(client) => tx.calculate_transaction(client),
                }
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        });
    println!("client,available,held,total,locked");
    for client in clients {
        println!(
            "{},{},{},{},{}",
            client.id,
            client.available.get_displayed_value(),
            client.held.get_displayed_value(),
            client.total.get_displayed_value(),
            client.locked
        );
    }
}

fn get_rdr() -> Reader<File> {
    let args: Vec<String> = env::args().collect();
    let transaction_path: &str = &args[1];
    ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(transaction_path)
        .unwrap()
}
