use crate::journal::accounting_tree::AccountTree;
use crate::journal::ledger::{Ledger, LedgerReader};
use chrono::{DateTime, Utc};

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

    //
    // Build the AccountTree by populating it with values from the ledger
    //    - Call this method build_tree or generate or execute etc
    // Consider adding methods to retrieve account trees with subtotals
    // Consider adding a method to return the IncomeStatement
    // Consider adding a method to return the CashflowStatement
    //
}
