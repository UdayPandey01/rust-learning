struct Transaction {
    id: u32,
    amount: f64,
    category: String,
}

fn main() {
    let transactions = vec![
        Transaction { id: 1, amount: 10.50, category: String::from("Groceries") },
        Transaction { id: 2, amount: 50.00, category: String::from("Electronics") },
        Transaction { id: 3, amount: 7.25, category: String::from("Groceries") },
        Transaction { id: 4, amount: 120.00, category: String::from("Apparel") },
        Transaction { id: 5, amount: 22.80, category: String::from("Groceries") },
    ];

    // TODO: Write a single, chained iterator expression.
    // 1. Start with an iterator over the `transactions` vector.
    // 2. Filter the transactions to keep only those in the "Groceries" category.
    // 3. Map the filtered transactions to just their `amount` values (which are f64s).
    // 4. Sum up all the amounts into a single f64 value.
    let total_grocery_spending: f64 = transactions
        .into_iter()
        .filter(|trans| trans.category == "Groceries")
        .map(|trans| trans.amount)
        .sum();

    println!("Total spending on groceries: ${:.2}", total_grocery_spending);
}