use crate::fixed_number::FixedNumber;
use crate::transactions::{Transaction, TransactionType};

#[derive(Debug, PartialEq)]
pub struct Client {
    pub id: u16,
    pub available: FixedNumber,
    pub held: FixedNumber,
    pub total: FixedNumber,
    pub locked: bool,
    transactions: Vec<Transaction>,
}

impl Client {
    fn add_transaction(&mut self, tx: &Transaction) {
        self.transactions.push(tx.clone());
    }

    fn calculate_total(&mut self) {
        self.total = FixedNumber::add(&self.held, &self.available);
    }

    pub fn deposit(&mut self, tx: &Transaction) {
        if let Some(amount) = &tx.amount {
            if amount.gt(0) {
                self.available = FixedNumber::add(&self.available, amount);
                self.add_transaction(tx);
                self.calculate_total();
            }
        }
    }

    pub fn withdraw(&mut self, tx: &Transaction) {
        if let Some(amount) = &tx.amount {
            if amount.gt(0) && self.available.gt_eq(amount) {
                self.available = FixedNumber::subtract(&self.available, amount);
                self.add_transaction(tx);
                self.calculate_total();
            }
        }
    }

    pub fn dispute(&mut self, tx: &Transaction) {
        let disputed_transaction = self
            .transactions
            .iter_mut()
            .find(|transaction| transaction.id == tx.id);
        match disputed_transaction {
            None => {}
            Some(disputed_transaction) => {
                disputed_transaction.mark_disputed();
                if let Some(amount) = &disputed_transaction.amount {
                    match disputed_transaction.transaction_type {
                        TransactionType::Deposit => {
                            self.available = FixedNumber::subtract(&self.available, amount);
                            self.held = FixedNumber::add(&self.held, amount);
                            self.calculate_total();
                        }
                        TransactionType::Withdrawal => {
                            self.held = FixedNumber::add(&self.available, amount);
                            self.calculate_total();
                        }
                        TransactionType::Dispute => {}
                        TransactionType::Resolve => {}
                        TransactionType::Chargeback => {}
                    }
                }
            }
        }
    }

    pub fn resolve(&mut self, tx: &Transaction) {
        let disputed_transaction = self
            .transactions
            .iter_mut()
            .find(|transaction| transaction.id == tx.id);
        match disputed_transaction {
            None => {}
            Some(disputed_transaction) => {
                if disputed_transaction.disputed {
                    disputed_transaction.mark_resolved();
                    if let Some(amount) = &disputed_transaction.amount {
                        self.available = FixedNumber::add(&self.available, amount);
                        self.held = FixedNumber::subtract(&self.held, amount);
                        self.calculate_total();
                    }
                }
            }
        }
    }

    pub fn chargeback(&mut self, tx: &Transaction) {
        let disputed_transaction = self
            .transactions
            .iter_mut()
            .find(|transaction| transaction.id == tx.id);

        match disputed_transaction {
            None => {}
            Some(disputed_transaction) => {
                if disputed_transaction.disputed {
                    disputed_transaction.mark_resolved();
                    if let Some(amount) = &disputed_transaction.amount {
                        self.held = FixedNumber::subtract(&self.held, amount);
                        self.locked = true;
                        self.calculate_total();
                    }
                }
            }
        }
    }
}

pub fn create_client(id: u16) -> Client {
    Client {
        id,
        available: FixedNumber::new(),
        held: FixedNumber::new(),
        total: FixedNumber::new(),
        locked: false,
        transactions: vec![],
    }
}

//TODO: Add a test that tries to resolve a transaction that isn't disputed
#[cfg(test)]
mod tests {
    use crate::fixed_number::FixedNumber;
    use crate::transactions::{Transaction, TransactionType};
    use crate::{create_client, Client};

    fn get_test_client() -> Client {
        Client {
            id: 0,
            available: FixedNumber::from_float(2.0),
            held: FixedNumber::new(),
            total: FixedNumber::from_float(2.0),
            locked: false,
            transactions: vec![Transaction {
                id: 0,
                client_id: 0,
                amount: Some(FixedNumber::from_float(2.0)),
                transaction_type: TransactionType::Deposit,
                disputed: false,
            }],
        }
    }

    fn get_withdrawal_test_client() -> Client {
        Client {
            id: 0,
            available: FixedNumber::new(),
            held: FixedNumber::new(),
            total: FixedNumber::new(),
            locked: false,
            transactions: vec![
                Transaction {
                    id: 0,
                    client_id: 0,
                    amount: Some(FixedNumber::from_float(2.0)),
                    transaction_type: TransactionType::Deposit,
                    disputed: false,
                },
                Transaction {
                    id: 1,
                    client_id: 0,
                    amount: Some(FixedNumber::from_float(2.0)),
                    transaction_type: TransactionType::Withdrawal,
                    disputed: false,
                },
            ],
        }
    }

    #[test]
    fn create_client_returns_default_client_with_specified_id() {
        let client_id: u16 = 1;
        let client = Client {
            id: client_id,
            available: FixedNumber::new(),
            held: FixedNumber::new(),
            total: FixedNumber::new(),
            locked: false,
            transactions: vec![],
        };
        assert_eq!(client, create_client(client_id));
    }

    #[test]
    fn negative_deposit_is_ignored() {
        let mut client = get_test_client();
        let negative_deposit = Transaction {
            id: 1,
            client_id: 0,
            amount: Some(FixedNumber::from_float(-2.0)),
            transaction_type: TransactionType::Deposit,
            disputed: false,
        };
        client.deposit(&negative_deposit);
        assert_eq!(get_test_client(), client);
    }

    #[test]
    fn negative_withdrawal_is_ignored() {
        let mut client = get_test_client();
        let negative_withdrawal = Transaction {
            id: 1,
            client_id: 0,
            amount: Some(FixedNumber::from_float(-2.0)),
            transaction_type: TransactionType::Withdrawal,
            disputed: false,
        };
        client.withdraw(&negative_withdrawal);
        assert_eq!(&get_test_client(), &client);
    }

    #[test]
    fn dispute_marks_amount_as_held_and_tx_as_disputed() {
        let mut client = get_test_client();
        let dispute = Transaction {
            id: 0,
            client_id: 0,
            amount: None,
            transaction_type: TransactionType::Dispute,
            disputed: false,
        };
        let expected_client = Client {
            id: 0,
            available: FixedNumber::new(),
            held: FixedNumber::from_float(2.0),
            total: FixedNumber::from_float(2.0),
            locked: false,
            transactions: vec![Transaction {
                id: 0,
                client_id: 0,
                amount: Some(FixedNumber::from_float(2.0)),
                transaction_type: TransactionType::Deposit,
                disputed: true,
            }],
        };
        client.dispute(&dispute);
        assert_eq!(expected_client, client);
    }

    #[test]
    fn dispute_marks_amount_as_held_and_tx_as_disputed_withdrawal() {
        let mut client = get_withdrawal_test_client();
        let dispute = Transaction {
            id: 1,
            client_id: 0,
            amount: None,
            transaction_type: TransactionType::Dispute,
            disputed: false,
        };
        let expected_client = Client {
            id: 0,
            available: FixedNumber::new(),
            held: FixedNumber::from_float(2.0),
            total: FixedNumber::from_float(2.0),
            locked: false,
            transactions: vec![
                Transaction {
                    id: 0,
                    client_id: 0,
                    amount: Some(FixedNumber::from_float(2.0)),
                    transaction_type: TransactionType::Deposit,
                    disputed: false,
                },
                Transaction {
                    id: 1,
                    client_id: 0,
                    amount: Some(FixedNumber::from_float(2.0)),
                    transaction_type: TransactionType::Withdrawal,
                    disputed: true,
                },
            ],
        };
        client.dispute(&dispute);
        assert_eq!(expected_client, client);
    }

    #[test]
    fn cannot_resolve_non_disputed_tx() {
        let mut client = get_test_client();
        let tx = Transaction {
            id: 0,
            client_id: 0,
            amount: Some(FixedNumber::from_float(2.0)),
            transaction_type: TransactionType::Deposit,
            disputed: false,
        };
        let expected_client = Client {
            id: 0,
            available: FixedNumber::from_float(2.0),
            held: FixedNumber::new(),
            total: FixedNumber::from_float(2.0),
            locked: false,
            transactions: vec![tx.clone()],
        };
        client.resolve(&tx);
        assert_eq!(expected_client, client);
    }

    #[test]
    fn resolving_dispute_unmarks_disputed_and_resets_balances() {
        let mut client = get_test_client();
        let dispute = Transaction {
            id: 0,
            client_id: 0,
            amount: None,
            transaction_type: TransactionType::Dispute,
            disputed: false,
        };
        let expected_client = Client {
            id: 0,
            available: FixedNumber::new(),
            held: FixedNumber::from_float(2.0),
            total: FixedNumber::from_float(2.0),
            locked: false,
            transactions: vec![Transaction {
                id: 0,
                client_id: 0,
                amount: Some(FixedNumber::from_float(2.0)),
                transaction_type: TransactionType::Deposit,
                disputed: true,
            }],
        };
        client.dispute(&dispute);
        assert_eq!(expected_client, client);
        let resolve = Transaction {
            id: 0,
            client_id: 0,
            amount: None,
            transaction_type: TransactionType::Resolve,
            disputed: false,
        };
        client.resolve(&resolve);
        let expected_client = Client {
            id: 0,
            available: FixedNumber::from_float(2.0),
            held: FixedNumber::new(),
            total: FixedNumber::from_float(2.0),
            locked: false,
            transactions: vec![Transaction {
                id: 0,
                client_id: 0,
                amount: Some(FixedNumber::from_float(2.0)),
                transaction_type: TransactionType::Deposit,
                disputed: false,
            }],
        };
        assert_eq!(expected_client, client);
    }

    #[test]
    fn chargeback_decreases_held_and_locks_user() {
        let mut client = Client {
            id: 0,
            available: FixedNumber::new(),
            held: FixedNumber::from_float(2.0),
            total: FixedNumber::from_float(2.0),
            locked: false,
            transactions: vec![Transaction {
                id: 0,
                client_id: 0,
                amount: Some(FixedNumber::from_float(2.0)),
                transaction_type: TransactionType::Deposit,
                disputed: true,
            }],
        };
        let chargeback = Transaction {
            id: 0,
            client_id: 0,
            amount: None,
            transaction_type: TransactionType::Chargeback,
            disputed: false,
        };
        client.chargeback(&chargeback);
        let expected_client = Client {
            id: 0,
            available: FixedNumber::new(),
            held: FixedNumber::new(),
            total: FixedNumber::new(),
            locked: true,
            transactions: vec![Transaction {
                id: 0,
                client_id: 0,
                amount: Some(FixedNumber::from_float(2.0)),
                transaction_type: TransactionType::Deposit,
                disputed: false,
            }],
        };
        assert_eq!(expected_client, client)
    }
}
