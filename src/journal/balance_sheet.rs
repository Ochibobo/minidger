use crate::journal::accounting_tree::{AccountTree, ActionType};
use crate::journal::ledger::{EntryType, Ledger, LedgerReader, TransactionEntry};
use chrono::{DateTime, Utc};
use std::borrow::Borrow;
use std::{collections::HashMap, rc::Rc};

///
/// `Balance Sheet` structure
/// This is basically a structured view of the ledger
///
pub struct BalanceSheet {
    id: usize,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    accounting_tree: AccountTree,
    ledger: Ledger,
}

// BalanceSheet implementation
impl BalanceSheet {
    pub fn new(
        id: usize,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
        accounting_tree: AccountTree,
        ledger: Ledger,
    ) -> Self {
        //todo!("Filter by from_date and to_date");

        BalanceSheet {
            id,
            from_date,
            to_date,
            accounting_tree,
            ledger,
        }
    }

    ///
    /// Create a new instance of the `BalanceSheet` by reading
    /// the ledger using a `LedgerReader`
    ///
    pub fn new_from_reader(
        id: usize,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
        accounting_tree: AccountTree,
        ledger_reader: impl LedgerReader,
    ) -> Self {
        // Get the ledger instance by reading it
        let ledger = ledger_reader.read_by_date_range(from_date, to_date);

        BalanceSheet {
            id,
            from_date,
            to_date,
            accounting_tree,
            ledger,
        }
    }

    ///
    /// Get the `BalanceSheet id``
    ///
    pub fn id(&self) -> usize {
        self.id
    }

    ///
    /// Set the `BalanceSheet id``
    ///
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    ///
    /// Set the `BalanceSheet from_date`
    ///
    pub fn set_from_date(&mut self, from_date: DateTime<Utc>) {
        self.from_date = from_date;
    }

    ///
    /// Get the `BalanceSheet from_date`
    ///
    pub fn from_date(&self) -> &DateTime<Utc> {
        &self.from_date
    }

    ///
    /// Set the `BalanceSheet to_date`
    ///
    pub fn set_to_date(&mut self, to_date: DateTime<Utc>) {
        self.to_date = to_date;
    }

    ///
    /// Get the `BalanceSheet to_date`
    ///
    pub fn to_date(&self) -> &DateTime<Utc> {
        &self.to_date
    }

    ///
    /// Set the `BalanceSheet account_tree`
    ///
    pub fn set_accounting_tree(&mut self, accounting_tree: AccountTree) {
        self.accounting_tree = accounting_tree;
    }

    ///
    /// Get the `BalanceSheet account_tree`
    ///
    pub fn accounting_tree(&self) -> &AccountTree {
        &self.accounting_tree
    }

    ///
    /// Set the `BalanceSheet Ledger`
    ///
    pub fn set_ledger(&mut self, ledger: Ledger) {
        _ = ledger;
        todo!("Check if this method is required");
        // self.ledger = ledger
    }

    ///
    /// Get the `BalanceSheet ledger`
    ///
    pub fn ledger(&self) -> &Ledger {
        &self.ledger
    }

    ///
    /// Build the AccountTree by populating it with values from the ledger
    ///    - Call this method build_tree or generate or execute etc
    ///    - Should return a Result<Ok, Err>
    ///
    pub fn build(&self) {
        // Retieve all transaction entries
        let transaction_entries: Vec<&Rc<TransactionEntry>> = self
            .ledger
            .journal_entries()
            .iter()
            .map(|journal_entry| journal_entry.transaction_entries())
            .flatten()
            .collect();

        // Group by account type
        // Collect all the unique account types by name
        let mut accounts_aggregate_map: HashMap<String, f64> = HashMap::new();

        // Perform aggregates for each account type taking credit/debit into consideration
        for transaction_entry in transaction_entries.iter() {
            // Perform the aggregations based on the Credit/Debit rule per account type
            let acc_type_ref = transaction_entry.account_type();
            let account_type_option = acc_type_ref.borrow().as_ref();
            let account_type;
            let acc_name = transaction_entry.account_name().clone();

            // Attempt to retieve the primary account type
            // This helps in knowing whether to increase/decrease the amount of an account
            // when the transaction is a credit/debit.
            match account_type_option {
                None => {
                    panic!(
                        "Transaction with id: {:?} and account name: {:?} has account_type of None",
                        transaction_entry.id(),
                        acc_name,
                    );
                }
                Some(account_type_result) => {
                    account_type = account_type_result.clone().to_owned();
                }
            }

            // Get the amount associated with the account name under the transaction entry or
            // initialize it to 0
            if !accounts_aggregate_map.contains_key(&acc_name) {
                accounts_aggregate_map.insert(acc_name.clone(), 0f64);
            }

            // Get the sign that'll be assigned to the amount
            let signum: f64;

            // Increase of decrease the amounts accordingly
            match transaction_entry.entry_type() {
                EntryType::Credit => match account_type.on_credit() {
                    ActionType::Decrease => {
                        signum = -1f64;
                    }
                    ActionType::Increase => {
                        signum = 1f64;
                    }
                },
                EntryType::Debit => match account_type.on_debit() {
                    ActionType::Decrease => {
                        signum = -1f64;
                    }
                    ActionType::Increase => {
                        signum = 1f64;
                    }
                },
            }

            // Apply the signum to the amount
            let transaction_amount = transaction_entry.amount() * signum;

            // Apply this amount delta in the accounts aggregate map
            accounts_aggregate_map.insert(
                acc_name.clone(),
                accounts_aggregate_map.get(&acc_name).unwrap() + transaction_amount,
            );
        }

        // Map the account name to an account tree node
        //  - Retrieve the tree node from a cache
        //  - In case of a miss, perform a DFS to retrieve the tree node
        //  - In case the account_name has no matching node, throw an error immeadiately
        // Use the aggregates to compute amounts that'll be propagated up the tree
        //  - Propagate the amounts up the tree.
        // Assert Assets = Liabilities + Equity
    }
    // Consider adding methods to retrieve account trees with subtotals
    // Consider adding a method to return the IncomeStatement
    // Consider adding a method to return the CashflowStatement
    //
}
