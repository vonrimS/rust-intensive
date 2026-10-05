use std::fmt::Display;

/// Custom account error.
#[derive(Debug, Clone, PartialEq)]
pub enum AccountError {
    InvalidAmount,
    InsufficientFunds { balance: f64, requested: f64 },
    AccountLocked,
}

impl Eq for AccountError {}

impl Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountError::InvalidAmount => write!(f, "Invalid amount"),
            AccountError::InsufficientFunds { balance, requested } => write!(
                f,
                "Insufficient funds: balance {}, requested {}",
                balance, requested
            ),
            AccountError::AccountLocked => write!(f, "Account is locked"),
        }
    }
}

impl std::error::Error for AccountError {}

/// Transaction kind (Deposit or Withdrawal).
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
}

/// Private transaction record.
#[derive(Debug, Clone, PartialEq)]
struct Transaction {
    id: usize,
    kind: TransactionKind,
    amount: f64,
}

/// Bank account entity with strictly private state.
pub struct BankAccount {
    account_number: String,
    balance: f64,
    is_locked: bool,
    transactions: Vec<Transaction>,
}

impl BankAccount {
    /// Creates a new bank account with an initial deposit.
    pub fn new(account_number: String, initial_deposit: f64) -> Result<Self, AccountError> {
        if initial_deposit <= 0.0 {
            return Err(AccountError::InvalidAmount);
        }

        Ok(BankAccount {
            account_number,
            balance: initial_deposit,
            is_locked: false,
            transactions: Vec::new(),
        })
    }

    /// Deposits funds into the accunt (if not locked).
    pub fn deposit(&mut self, amount: f64) -> Result<f64, AccountError> {
        if self.is_locked {
            return Err(AccountError::AccountLocked);
        }

        if amount <= 0.0 {
            return Err(AccountError::InvalidAmount);
        }

        self.balance += amount;
        self.transactions.push(Transaction {
            id: self.transactions.len() + 1,
            kind: TransactionKind::Deposit,
            amount,
        });

        Ok(self.balance)
    }

    /// Withdraws funds from the account (if not locked and sufficient funds exist).
    pub fn withdraw(&mut self, amount: f64) -> Result<f64, AccountError> {
        if self.is_locked {
            return Err(AccountError::AccountLocked);
        }

        if amount <= 0.0 {
            return Err(AccountError::InvalidAmount);
        }

        if self.balance < amount {
            return Err(AccountError::InsufficientFunds {
                balance: self.balance,
                requested: amount,
            });
        }

        self.balance -= amount;
        self.transactions.push(Transaction {
            id: self.transactions.len() + 1,
            kind: TransactionKind::Withdrawal,
            amount,
        });

        Ok(self.balance)
    }

    /// Read-only balance accessor.
    pub fn balance(&self) -> f64 {
        self.balance
    }

    /// Safe optional access to the most recent transaction log.
    pub fn last_transaction(&self) -> Option<&Transaction> {
        self.transactions.last()
    }

    /// Locks the account, rejecting future state mutations.
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    /// Unlocks the account.
    pub fn unlock(&mut self) {
        self.is_locked = false;
    }
}
