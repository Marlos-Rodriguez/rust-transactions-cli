use serde::Deserialize;
/// Implementation for basic transactions in CSV
/// This is focused on processing CSV files
/// ```
/// type,      client,  tx, amount
/// deposit,        1,   1,    1.0
/// withdrawal,     1,   4,    1.5
/// ```
///
/// # Example
/// You can create a transaction from a [`csv::StringRecord`] with [`Transaction::new_from_csv`]:
/// ```
/// let mut rdr = csv::Reader::from_reader(tx.as_bytes());
/// for result in rdr.records() {
///     let tx = Transaction::new_from_csv(result.unwrap());
/// }
/// ```
#[derive(Debug, PartialEq, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub client: u32,
    pub tx: u32,
    pub amount: f32,
}

impl Transaction {
    pub fn is_dispute(&self) -> bool {
        match self.tx_type.as_str() {
            "dispute" => true,
            "resolve" => true,
            "chargeback" => true,
            _ => false,
        }
    }

    /// Returns the amount to change of this [`Transaction`].
    /// This assume that the only transaction types are "deposit" or "withdrawal".
    /// # Examples
    /// It's use for clients implementation [`Client::process_transactions(transactions);`]:
    /// ```
    /// let change: f32 = transaction.get_amount_change();
    /// client = client.new_transaction(change);
    /// ```
    pub fn get_amount_change(&self) -> f32 {
        match self.tx_type.as_str() {
            "deposit" => self.amount,
            "withdrawal" => self.amount * -1.0,
            _ => 0.0,
        }
    }

    /// Return the transactions from a csv given the path as parameter.
    /// it's assume that with no further arguments the transactions in the CSV is sorted by the ID
    /// ```
    /// transactions.sort_by_key(|a| a.tx);
    /// ```
    /// # Panics
    ///
    /// Panics if the path is invalid.
    /// # Examples
    /// ```
    /// let args: Vec<String> = env::args().collect();
    /// let path: &String = &args[1];
    /// let transactions: Vec<Transaction> = Transaction::get_transactions(path);
    /// ```
    pub fn get_transactions(tx: String) -> Vec<Transaction> {
        let mut transactions: Vec<Transaction> = Vec::new();

        /* let mut tx_csv = csv::Reader::from_reader(tx.as_bytes()).flexible_reader(); */
        let mut tx_csv = csv::ReaderBuilder::new()
            .flexible(true)
            .from_reader(tx.as_bytes());
        for result in tx_csv.records() {
            let record = Transaction::new_from_csv(result.unwrap());
            transactions.push(record);
        }

        //This part assume that with no further arguments the transactions in the CSV is sorted by the ID
        transactions.sort_by_key(|a| a.tx);

        return transactions;
    }

    /// Get the transaction index from a vec of transactions.
    /// Search one with the same ID and deposit like transaction type.
    ///
    /// # Examples
    /// ```
    /// Transaction::get_prev_trans(txs, transaction.tx)
    /// ```
    pub fn get_prev_trans(txs: &Vec<Transaction>, tx_id: u32) -> Option<usize> {
        txs.iter()
            .position(|tx| tx.tx == tx_id && tx.tx_type.as_str() == "deposit")
    }

    pub fn new(tx_type: String, client: u32, tx: u32, amount: f32) -> Self {
        Self {
            tx_type,
            client,
            tx,
            amount,
        }
    }

    /// Generate a new transaction from a [`csv::StringRecord`]
    ///
    /// # Example
    /// You can create a transaction from  [`csv::StringRecord`] with [`Transaction::new_from_csv`]:
    /// ```
    /// let mut rdr = csv::Reader::from_reader(tx.as_bytes());
    /// for result in rdr.records() {
    ///     let tx = Transaction::new_from_csv(result.unwrap());
    /// }
    /// ```
    pub fn new_from_csv(sr: csv::StringRecord) -> Self {
        let tx_type: String = sr.get(0).unwrap().to_string();
        let client: u32 = sr.get(1).unwrap().trim().parse::<u32>().unwrap();
        let tx: u32 = sr.get(2).unwrap().trim().parse::<u32>().unwrap();
        let amount: f32 = match sr.get(3) {
            Some(a) => a.trim().parse::<f32>().unwrap(),
            None => 0.0,
        };

        Self {
            tx_type,
            client,
            tx,
            amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_transaction_test() {
        let sr: csv::StringRecord = csv::StringRecord::from(vec!["deposit", "1", "1", "1.0"]);
        let tx_csv: Transaction = Transaction::new_from_csv(sr);
        let tx: Transaction = Transaction {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: 1.0,
        };
        assert_eq!(tx, tx_csv);
    }

    #[test]
    fn amount_test() {
        let tx: Transaction = Transaction {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: 1.0,
        };

        assert_eq!(tx.get_amount_change(), 1.0)
    }

    #[test]
    fn get_transactions_test() {
        let tx_string: String = String::from("type, client, tx, amount\ndeposit, 1, 1, 1.0");
        let tx_csv: Vec<Transaction> = Transaction::get_transactions(tx_string);

        let tx: Transaction = Transaction {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: 1.0,
        };
        let txs: Vec<Transaction> = vec![tx];

        assert_eq!(txs, tx_csv);
    }
}
