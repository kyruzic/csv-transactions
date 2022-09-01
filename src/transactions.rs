use crate::clients::Client;
use crate::fixed_number::FixedNumber;
use core::result::Result::{Err, Ok};
use serde::Deserialize;
use std::str::FromStr;

pub fn format_transaction(tx: InputTransaction) -> Result<Transaction, String> {
    let transaction_type = TransactionType::from_str(&tx.transaction_type);
    match transaction_type {
        Ok(transaction_type) => Ok(Transaction {
            client_id: tx.client_id,
            id: tx.transaction_id,
            amount: tx.amount.map(FixedNumber::from_float),
            transaction_type,
            disputed: false,
        }),
        Err(e) => Err(e),
    }
}

#[derive(Debug, Deserialize)]
pub struct InputTransaction {
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    transaction_id: u32,
    amount: Option<f64>,
    #[serde(rename = "type")]
    transaction_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: u32,
    pub client_id: u16,
    pub amount: Option<FixedNumber>,
    pub transaction_type: TransactionType,
    pub disputed: bool,
}

impl Transaction {
    pub fn calculate_transaction(&self, client: &mut Client) {
        match self.transaction_type {
            TransactionType::Deposit => client.deposit(self),
            TransactionType::Withdrawal => client.withdraw(self),
            TransactionType::Dispute => client.dispute(self),
            TransactionType::Resolve => client.resolve(self),
            TransactionType::Chargeback => client.chargeback(self),
        };
    }

    pub fn mark_disputed(&mut self) {
        self.disputed = true;
    }

    pub fn mark_resolved(&mut self) {
        self.disputed = false;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deposit" => Ok(TransactionType::Deposit),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "dispute" => Ok(TransactionType::Dispute),
            "resolve" => Ok(TransactionType::Resolve),
            "chargeback" => Ok(TransactionType::Chargeback),
            _ => Err("The transaction type is invalid".to_string()),
        }
    }
}
