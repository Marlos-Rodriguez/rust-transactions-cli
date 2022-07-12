# Transactions CSV CLI

A simple CLI that read a CSV file and return a list of users with the final balance serilized for CSV as well.

## Use

The input is a CSV file with this headers

```
type, client, tx, amount
```

The path is passed in the arguments

```bash
cargo run -- transactions.csv
```

This returns a standard output:

```
client,available,held,total,locked
```

For saving the response in a CSV file run:

```
cargo run -- transactions.csv > accounts.csv
```
