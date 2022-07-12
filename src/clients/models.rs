use super::super::transactions::models::Transaction;
use serde::Serialize;

/// Implementation of Client for CSV
/// # Examples
/// To create a new Client you can use [`Client::new(client_id)`]:
/// ```
/// let mut new_client: Client = Client::new(client_id);
/// ```
///
/// Also a Client is created when the transactions are processed and a new Client ID is found.
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub struct Client {
    client: u32,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

impl Client {
    /// Returns a Serialize String with all the users
    pub fn clients_to_csv(clients: Vec<Client>) -> String {
        let mut clients_csv: csv::Writer<Vec<u8>> = csv::Writer::from_writer(vec![]);

        for client in clients {
            if let Err(_err) = clients_csv.serialize(&client) {
                panic!(r#"Error serializing"#);
            }
        }

        let data: String = String::from_utf8(clients_csv.into_inner().unwrap()).unwrap();
        return data;
    }

    /// Create a new empty client, ID is required.
    pub fn new(client: u32) -> Self {
        Self {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    /// Process the transaction depending on the of the type
    /// Update the value of the user and return the object.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use accounts_cli::clients::models::Client;
    ///
    /// let mut client: Client = Client::new(client_id);
    /// client = client.new_transaction("deposit", 1.0);
    /// assert_eq!(client.total, 1.0);
    /// ```
    pub fn new_transaction(mut self, tx_type: String, amount: f32) -> Self {
        match tx_type.as_str() {
            "deposit" => {
                self.available += amount;
            }
            "withdrawal" => {
                if (self.available - amount) > 0.0 {
                    self.available -= amount;
                }
            }
            "dispute" => {
                self.available -= amount;
                self.held += amount;
            }
            "resolve" => {
                self.available += self.held;
                self.held -= amount;
            }
            "chargeback" => {
                self.held -= amount;
                self.locked = true;
            }
            _ => {}
        }

        self.total = self.available + self.held;

        self
    }

    /// Process All transactions and return the client with the balance.
    /// For no further information about client, this create a new one when a new ID is found.
    ///
    /// # Examples
    /// ```
    /// let transactions: Vec<Transaction> = Transaction::get_transactions(path);
    /// let clients: Vec<Client> = Client::process_transactions(transactions);
    /// ```
    pub fn process_transactions(txs: &Vec<Transaction>) -> Vec<Client> {
        let mut clients: Vec<Client> = Vec::new();

        for transaction in txs {
            let client_id: u32 = transaction.client;

            //If the client exists
            match clients.iter().position(|c| c.client == client_id) {
                Some(cl_index) => {
                    if clients[cl_index].locked {
                        continue;
                    }

                    //If the transaction is a dispute, the previos amount need to be found
                    if transaction.is_dispute() {
                        match Transaction::get_prev_trans(txs, transaction.tx) {
                            Some(ori_tx_id) => {
                                //If the previos tx exists, make the transaction.
                                clients[cl_index] = clients[cl_index].new_transaction(
                                    transaction.tx_type.clone(),
                                    txs[ori_tx_id].amount,
                                );
                                continue;
                            }
                            _ => (),
                        }
                    }

                    //If the transaction is not a dispute, the amount of tx is used
                    clients[cl_index] = clients[cl_index]
                        .new_transaction(transaction.tx_type.clone(), transaction.amount);
                }
                //If the user don't exists, create a new one and make the transaction.
                None => {
                    let mut new_client: Client = Client::new(client_id);
                    new_client =
                        new_client.new_transaction(transaction.tx_type.clone(), transaction.amount);

                    clients.push(new_client);
                }
            }
        }

        return clients;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let cl: Client = Client {
            client: 0,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };
        let new_cl: Client = Client::new(0);
        assert_eq!(cl, new_cl)
    }

    #[test]
    fn new_transaction_test() {
        let new_cl: Client = Client::new(0);

        assert_eq!(
            new_cl.new_transaction("deposit".to_string(), 1.2).available,
            1.2
        );

        let cl_withdraw: Client = Client::new(1);
        assert_eq!(
            cl_withdraw
                .new_transaction("withdrawal".to_string(), 1.0)
                .available,
            0.0
        );

        let cl_dispute: Client = Client::new(2);
        assert_eq!(
            cl_dispute.new_transaction("dispute".to_string(), 1.0).held,
            1.0
        );

        //In this part create a scenario when a resolve can happen:
        //A deposit
        let mut cl_to_resolve: Client = cl_dispute.new_transaction("deposit".to_string(), 1.0);
        //Then a dispute
        cl_to_resolve = cl_to_resolve.new_transaction("dispute".to_string(), 1.0);
        //To finally test the resolve.
        assert_eq!(
            cl_to_resolve
                .new_transaction("resolve".to_string(), 1.0)
                .available,
            1.0
        );

        let cl_dispute_cb: Client = Client::new(3);
        let cl_chargeback = cl_dispute_cb.new_transaction("dispute".to_string(), 1.0);
        assert_eq!(
            cl_chargeback
                .new_transaction("chargeback".to_string(), 1.0)
                .held,
            0.0
        );

        assert_eq!(
            cl_chargeback
                .new_transaction("chargeback".to_string(), 1.0)
                .locked,
            true
        );
    }

    #[test]
    fn process_transactions_test() {
        let tx: Transaction = Transaction::new("deposit".to_string(), 1, 1, 1.0);
        let txs: Vec<Transaction> = vec![tx];

        let new_cl: Client = Client::new(1);

        let clients: Vec<Client> = Client::process_transactions(&txs);

        assert_eq!(clients[0].client, new_cl.client)
    }

    #[test]
    fn clients_csv_test() {
        let client = Client::new(1);
        let clients: Vec<Client> = vec![client];

        let cl_string: String = Client::clients_to_csv(clients);

        let clients_string: String =
            String::from("client,available,held,total,locked\n1,0.0,0.0,0.0,false\n");

        assert_eq!(cl_string, clients_string)
    }
}
