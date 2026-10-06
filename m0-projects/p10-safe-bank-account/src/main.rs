use p10_safe_bank_account::{BankAccount, TransactionKind};

fn main() {
    println!("=== Safe Bank Account Management System ===\n");

    let acc_num = "ACC-1001";
    let init_deposit = 100.0;
    let mut account =
        BankAccount::new(acc_num.to_string(), init_deposit).expect("Failed to create account");

    println!(
        "[+] Created account: {} with initial deposit: ${:.2}",
        acc_num, init_deposit
    );

    println!("Current Balance: ${}\n", account.balance());

    let deposited = 50.0;
    let withdraw = 30.0;

    println!("--- Executing Transactions ---");
    if let Ok(new_bal) = account.deposit(deposited) {
        println!(
            "[+] Deposited: ${:.2} | New Balance: ${:.2}",
            deposited, new_bal,
        );
    }

    if let Ok(new_bal) = account.withdraw(withdraw) {
        println!(
            "[+] Withdrew:  ${:.2} | New Balance: ${:.2}",
            withdraw, new_bal,
        );
    }

    println!();

    println!("--- Inspecting Last Transaction ---");

    if let Some(tx) = account.last_transaction() {
        let action = match tx.kind {
            TransactionKind::Deposit => "Deposit",
            TransactionKind::Withdrawal => "Withdrawal",
        };
        println!("Last Action: {} of ${:.2}\n", action, tx.amount);
    }

    println!("--- Boundary & Error Checks ---");
    println!("[!] Attempting invalid deposit ($0.00):");
    if let Err(err) = account.deposit(0.0) {
        println!("Error: {:?}", err);
    }
    println!();

    println!("[!] Attempting overdraft withdrawal ($200.00):");
    if let Err(err) = account.withdraw(200.0) {
        println!("Error: {:?}", err);
    }
}
