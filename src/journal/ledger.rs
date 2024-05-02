use crate::journal::accounting_tree::AccountNodeRef;
use chrono::{DateTime, Utc};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum EntryType {
    Credit,
    Debit,
}

///
/// `TransactionEntry` is a single row entry that makes up a JournalEntry
///
#[derive(Debug)]
pub struct TransactionEntry {
    account: AccountNodeRef,
    amount: f64,
    entry_type: EntryType,
    date_of_entry: DateTime<Utc>,
    description: String,
}

impl TransactionEntry {
    pub fn new(
        account: AccountNodeRef,
        amount: f64,
        entry_type: EntryType,
        date_of_entry: DateTime<Utc>,
        description: &str,
    ) -> Self {
        TransactionEntry {
            account,
            amount,
            entry_type,
            date_of_entry,
            description: description.to_owned(),
        }
    }

    pub fn account(&self) -> AccountNodeRef {
        self.account.clone()
    }

    pub fn set_account(&mut self, account: AccountNodeRef) {
        self.account = account
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: f64) {
        self.amount = amount
    }

    pub fn entry_type(&self) -> &EntryType {
        &self.entry_type
    }

    pub fn set_entry_type(&mut self, entry_type: EntryType) {
        self.entry_type = entry_type
    }

    pub fn date_of_entry(&self) -> &DateTime<Utc> {
        &self.date_of_entry
    }

    pub fn set_date_of_entry(&mut self, date_of_entry: DateTime<Utc>) {
        self.date_of_entry = date_of_entry
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_owned()
    }
}

///
/// `JournalEntry` structure that holds a set of related `TransactionEntries`.
/// The sum of the `Credit` entries must equal the `Debit` entries.
///
#[derive(Debug)]
pub struct JournalEntry {
    id: usize,
    transaction_entries: Vec<Rc<TransactionEntry>>,
    date_of_entry: DateTime<Utc>,
    description: String,
}

impl JournalEntry {
    pub fn new(id: usize, date_of_entry: DateTime<Utc>, description: &str) -> Self {
        let transaction_entries: Vec<Rc<TransactionEntry>> = Vec::new();
        JournalEntry {
            id,
            transaction_entries,
            date_of_entry,
            description: description.to_owned(),
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id
    }

    pub fn set_date_of_entry(&mut self, date_of_entry: DateTime<Utc>) {
        self.date_of_entry = date_of_entry
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description
    }

    pub fn add_transaction_entry(&mut self, transaction_entry: Rc<TransactionEntry>) {
        self.transaction_entries.push(transaction_entry);
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn date_of_entry(&self) -> DateTime<Utc> {
        self.date_of_entry
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn transaction_entries(&self) -> &Vec<Rc<TransactionEntry>> {
        &self.transaction_entries
    }

    pub fn number_of_transaction_entries(&self) -> usize {
        self.transaction_entries().len()
    }

    pub fn total_credit(&self) -> f64 {
        let total_credits = self
            .transaction_entries()
            .into_iter()
            .filter(|entry| entry.entry_type() == &EntryType::Credit)
            .map(|credit_entry| credit_entry.amount())
            .sum();

        return total_credits;
    }

    pub fn total_debit(&self) -> f64 {
        let total_debits = self
            .transaction_entries()
            .into_iter()
            .filter(|entry| entry.entry_type() == &EntryType::Debit)
            .map(|credit_entry| credit_entry.amount())
            .sum();

        return total_debits;
    }

    // Should return a Result<Ok(), JournalEntryError()>
    pub fn validate(&self) -> bool {
        let mut debits: f64 = 0.0;
        let mut credits: f64 = 0.0;

        for entry in self.transaction_entries() {
            match entry.entry_type() {
                // Sum the credit amounts
                EntryType::Credit => {
                    credits += entry.amount();
                }

                // Sum the debit amounts
                EntryType::Debit => {
                    debits += entry.amount();
                }
            }
        }

        return debits == credits;
    }
}

///
/// `General Ledger` that comprises of a set of journal entries.
/// This is the structure that feeds into the `balance sheet`, the `income statement`
/// and the `statemement of cashflow`.
///
#[derive(Debug)]
pub struct Ledger {
    id: usize,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    journal_entries: Vec<JournalEntry>,
}

impl Ledger {
    pub fn new(id: usize, from_date: DateTime<Utc>, to_date: DateTime<Utc>) -> Self {
        Ledger {
            id,
            from_date,
            to_date,
            journal_entries: Vec::new(),
        }
    }

    ///
    /// Get the `Ledger` id
    ///
    pub fn id(&self) -> &usize {
        &self.id
    }

    ///
    /// Set the `Ledger`'s id
    ///
    pub fn set_id(&mut self, id: usize) {
        self.id = id
    }

    ///
    /// Set the `from_date` - the date when the ledger entries begin
    ///
    pub fn set_from_date(&mut self, from_date: DateTime<Utc>) {
        self.from_date = from_date;
    }

    ///
    /// Get the `Ledger's` from_date
    ///
    pub fn from_date(&self) -> &DateTime<Utc> {
        &self.from_date
    }

    ///
    /// Set the `to_date` - the date when the ledger entries end
    ///
    pub fn set_to_date(&mut self, to_date: DateTime<Utc>) {
        self.to_date = to_date;
    }

    ///
    /// Get the `Ledger's` to_date
    ///
    pub fn to_date(&self) -> &DateTime<Utc> {
        &self.to_date
    }

    ///
    /// Used to validate that the dates of the journal entry are in sync
    /// with the dates of the ledger
    ///
    fn validate_journal_entry_dates(&self, journal_entry: &JournalEntry) {
        assert!(&journal_entry.date_of_entry() >= self.from_date());
        assert!(&journal_entry.date_of_entry() <= self.to_date());
    }

    ///
    /// Add a single journal entry
    ///
    pub fn add_journal_entry(&mut self, journal_entry: JournalEntry) {
        self.validate_journal_entry_dates(&journal_entry);
        self.journal_entries.push(journal_entry);
    }

    ///
    /// Add multiple journal entries
    ///
    pub fn add_journal_entries(&mut self, journal_entries: &mut Vec<JournalEntry>) {
        journal_entries
            .iter()
            .for_each(|entry| self.validate_journal_entry_dates(entry));

        self.journal_entries.append(journal_entries);
    }

    ///
    /// Replace all journal entries with the new one
    ///
    pub fn set_journal_entries(&mut self, journal_entries: Vec<JournalEntry>) {
        journal_entries
            .iter()
            .for_each(|entry| self.validate_journal_entry_dates(entry));

        self.journal_entries = journal_entries;
    }

    ///
    /// `Remove` a `journal entry` from the `ledger`
    ///
    pub fn remove_journal_entry(&mut self, id: usize) {
        self.journal_entries.retain(|j| j.id() != id)
    }

    ///
    /// `Remove`` all `journal entries` from the `ledger`
    ///
    pub fn remove_all_journal_entries(&mut self) {
        self.journal_entries.clear()
    }

    ///
    /// Remove all journal entries and set the id to `0`
    ///
    pub fn reset(&mut self) {
        self.set_id(0);
        self.remove_all_journal_entries();
    }

    ///
    /// Get all journal entries
    ///
    pub fn journal_entries(&self) -> &Vec<JournalEntry> {
        &self.journal_entries
    }

    ///
    /// Get a journal entry by id
    ///
    pub fn get_journal_entry_by_id(&self, id: usize) -> Option<&JournalEntry> {
        self.journal_entries.iter().find(|j| j.id() == id)
    }

    ///
    /// Get a journal entry by date of entry
    ///
    pub fn get_journal_entries_by_date(&self, date_of_entry: DateTime<Utc>) -> Vec<&JournalEntry> {
        self.journal_entries
            .iter()
            .filter(|j| j.date_of_entry == date_of_entry)
            .into_iter()
            .collect()
    }

    ///
    /// Get journal entries by date range (date of entry range)
    ///
    pub fn get_journal_entry_by_between(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Vec<&JournalEntry> {
        self.journal_entries
            .iter()
            .filter(|j| j.date_of_entry >= start_date && j.date_of_entry <= end_date)
            .into_iter()
            .collect()
    }

    ///
    /// Get a journal entries by description
    ///
    pub fn get_journal_entry_by_description(&self, description: &str) -> Vec<&JournalEntry> {
        self.journal_entries
            .iter()
            .filter(|j| j.description.contains(description))
            .into_iter()
            .collect()
    }

    ///
    /// Get the `number of journal entries` in the `ledger`
    ///
    pub fn number_of_journal_entries(&self) -> usize {
        self.journal_entries.len()
    }
}

#[cfg(test)]
mod test {
    use crate::journal::accounting_tree::{
        AccountNode, AccountNodeRef, AccountTagNode, AccountTreeNode, ActionType, ParentNodeT,
        PrimaryAccountType, RootNode, RootNodeRef,
    };

    use super::EntryType;
    use super::JournalEntry;
    use super::TransactionEntry;
    use chrono::Utc;
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    fn get_account_nodes_map() -> HashMap<String, AccountNodeRef> {
        let root: RootNodeRef = Rc::new(RefCell::new(RootNode::new()));

        let asset: Rc<PrimaryAccountType> = Rc::new(PrimaryAccountType::new(
            "Assets",
            ActionType::Increase,
            ActionType::Decrease,
        ));

        let liabilities: Rc<PrimaryAccountType> = Rc::new(PrimaryAccountType::new(
            "Liabilities",
            ActionType::Decrease,
            ActionType::Increase,
        ));

        let equity: Rc<PrimaryAccountType> = Rc::new(PrimaryAccountType::new(
            "Owner's Equity",
            ActionType::Increase,
            ActionType::Decrease,
        ));

        let asset_node = Rc::new(RefCell::new(AccountTagNode::new(
            1,
            "Asset",
            Some(root.clone()),
            Some(asset.clone()),
        )));

        let liabilities_node = Rc::new(RefCell::new(AccountTagNode::new(
            1,
            "Liabilities",
            Some(root.clone()),
            Some(liabilities.clone()),
        )));

        let equity_node = Rc::new(RefCell::new(AccountTagNode::new(
            1,
            "Owner's Equity",
            Some(root.clone()),
            Some(equity.clone()),
        )));

        {
            let mut root_ref = root.as_ref().borrow_mut();
            root_ref.add_child(asset_node.clone());
            root_ref.add_child(liabilities_node.clone());
            root_ref.add_child(equity_node.clone());
        }

        let current_assets_node = Rc::new(RefCell::new(AccountTagNode::new(
            2,
            "Current Assets",
            Some(asset_node.clone()),
            None,
        )));

        let current_liabilities_node = Rc::new(RefCell::new(AccountTagNode::new(
            2,
            "Current Liabilities",
            Some(liabilities_node.clone()),
            None,
        )));

        let retained_earnings_node = Rc::new(RefCell::new(AccountTagNode::new(
            3,
            "Retained Earnings",
            Some(equity_node.clone()),
            None,
        )));

        // Necessary to drop the mutable borrowed reference
        {
            let mut asset_n = asset_node.as_ref().borrow_mut();
            asset_n.add_child(current_assets_node.clone());

            let mut equity_n = equity_node.as_ref().borrow_mut();
            equity_n.add_child(retained_earnings_node.clone());

            let mut liabilities_n = liabilities_node.as_ref().borrow_mut();
            liabilities_n.add_child(current_liabilities_node.clone());
        }

        // An AccountNode's definition example
        let cash = Rc::new(RefCell::new(AccountNode::new(
            3,
            "Cash",
            Some(current_assets_node.clone()),
        )));

        let inventory = Rc::new(RefCell::new(AccountNode::new(
            3,
            "Inventory",
            Some(current_assets_node.clone()),
        )));

        // The accounts payable node
        let short_term_loan = Rc::new(RefCell::new(AccountNode::new(
            3,
            "Short Term Loan",
            Some(current_liabilities_node.clone()),
        )));

        // Revenue and cost of sales nodes
        let revenue = Rc::new(RefCell::new(AccountNode::new(
            3,
            "Revenue",
            Some(retained_earnings_node.clone()),
        )));

        let cost_of_sales = Rc::new(RefCell::new(AccountNode::new(
            3,
            "Cost of Sales",
            Some(retained_earnings_node.clone()),
        )));

        {
            let mut current_asset_n = current_assets_node.as_ref().borrow_mut();
            current_asset_n.add_child(cash.clone());
            current_asset_n.add_child(inventory.clone());

            let mut retained_earnings_n = retained_earnings_node.as_ref().borrow_mut();
            retained_earnings_n.add_child(revenue.clone());
            retained_earnings_n.add_child(cost_of_sales.clone());

            let mut current_liabilities_n = current_liabilities_node.as_ref().borrow_mut();
            current_liabilities_n.add_child(short_term_loan.clone());
        }

        let mut accounts_map = HashMap::new();

        accounts_map.insert("cash".to_owned(), cash.clone());
        accounts_map.insert("inventory".to_owned(), inventory.clone());
        accounts_map.insert("short_term_loan".to_owned(), short_term_loan.clone());
        accounts_map.insert("revenue".to_owned(), revenue.clone());
        accounts_map.insert("cost_of_sales".to_owned(), cost_of_sales.clone());

        return accounts_map.to_owned();
    }

    ///
    /// Test the creation of a transaction entry
    ///
    #[test]
    fn test_transaction_entry_creation() {
        let account_nodes_map = get_account_nodes_map();
        let cash_account = account_nodes_map.get("cash").unwrap().to_owned();
        let grocery_transaction_entry = TransactionEntry::new(
            cash_account.clone(),
            1_000.00,
            EntryType::Debit,
            Utc::now(),
            "Incoming investment",
        );

        assert_eq!(grocery_transaction_entry.amount(), 1_000.00);
        assert_eq!(grocery_transaction_entry.entry_type(), &EntryType::Debit);
        assert!(grocery_transaction_entry
            .description()
            .eq("Incoming investment"));

        let cloned_account = grocery_transaction_entry.account().clone();
        let retrieved_account = cloned_account.as_ref().borrow();

        assert_eq!(retrieved_account.name(), "Cash");

        let acc_type = retrieved_account.account_type().as_ref().unwrap();
        let primary_account_name = acc_type.name();

        assert_eq!(primary_account_name, "Assets");
    }

    ///
    /// Test the creation of a journal entry.
    ///
    /// This is the scenario being modelled:
    ///     - Took a short term loan worth Kshs. 400.00.
    ///         - Short Term Loan
    ///         - Cash
    ///     - Purchased inventory worth Kshs. 400.00.
    ///         - Cash
    ///         - Inventory
    ///     - Then sold the inventory for Kshs. 700.00.
    ///         - Inventory
    ///         - Cash
    ///         - Revenue
    ///         - Cost of Sale
    ///
    #[test]
    fn test_journal_entry_creation() {
        // Get the accounts node map instance
        let account_nodes_map = get_account_nodes_map();

        // Transaction entries for the loan amount
        let short_term_loan_node = account_nodes_map.get("short_term_loan").unwrap().to_owned();
        let cash_node = account_nodes_map.get("cash").unwrap().to_owned();
        let inventory_node = account_nodes_map.get("inventory").unwrap().to_owned();
        let revenue_node = account_nodes_map.get("revenue").unwrap().to_owned();
        let cost_of_sales_node = account_nodes_map.get("cost_of_sales").unwrap().to_owned();

        // First journal entry
        let mut journal_entry =
            JournalEntry::new(1, Utc::now(), "Entry for loan used to purchase inventory");

        // Short term loan
        let loan_entry = Rc::new(TransactionEntry::new(
            short_term_loan_node.clone(),
            400.00,
            EntryType::Credit,
            Utc::now(),
            "Short-term loan to purchase inventory",
        ));

        // Cash entry increase from this loan
        let cash_entry_from_loan = Rc::new(TransactionEntry::new(
            cash_node.clone(),
            400.00,
            EntryType::Debit,
            Utc::now(),
            "Cash that came from the inventory loan",
        ));

        journal_entry.add_transaction_entry(loan_entry.clone());
        journal_entry.add_transaction_entry(cash_entry_from_loan.clone());

        assert_eq!(journal_entry.total_credit(), 400.00);
        assert_eq!(journal_entry.total_debit(), 400.00);
        assert_eq!(journal_entry.number_of_transaction_entries(), 2);

        let cash_for_inventory_purchase = Rc::new(TransactionEntry::new(
            cash_node.clone(),
            400.00,
            EntryType::Credit,
            Utc::now(),
            "Cash used to purchase inventory",
        ));

        let inventory_purchased = Rc::new(TransactionEntry::new(
            inventory_node.clone(),
            400.00,
            EntryType::Debit,
            Utc::now(),
            "Inventory to be purchased",
        ));

        journal_entry.add_transaction_entry(cash_for_inventory_purchase.clone());
        journal_entry.add_transaction_entry(inventory_purchased.clone());

        assert!(journal_entry.validate());

        let mut sale_journal_entry = JournalEntry::new(
            2,
            Utc::now(),
            "Journal entry for the sale of the inventory.",
        );

        let inventory_sale = Rc::new(TransactionEntry::new(
            inventory_node.clone(),
            400.00,
            EntryType::Credit,
            Utc::now(),
            "Selling the purchased inventory",
        ));

        let cash_from_sale = Rc::new(TransactionEntry::new(
            cash_node.clone(),
            700.00,
            EntryType::Debit,
            Utc::now(),
            "Cash received from the sale of the inventory",
        ));

        sale_journal_entry.add_transaction_entry(inventory_sale.clone());
        sale_journal_entry.add_transaction_entry(cash_from_sale.clone());

        // This journal entry oughts to be false here
        assert_eq!(sale_journal_entry.validate(), false);

        // The Debit should exceed the credit by 300.00/=
        let total_credit = sale_journal_entry.total_credit();
        let total_debit = sale_journal_entry.total_debit();

        assert_ne!(total_debit, total_credit);
        assert_eq!(total_debit - total_credit, 300.00);

        // Record the revenue and cost of sale
        let revenue = Rc::new(TransactionEntry::new(
            revenue_node.clone(),
            700.00,
            EntryType::Credit,
            Utc::now(),
            "Revenue from the sale of the inventory",
        ));

        let cost_of_sales = Rc::new(TransactionEntry::new(
            cost_of_sales_node.clone(),
            400.00,
            EntryType::Debit,
            Utc::now(),
            "Cost equivalent to selling the inventory",
        ));

        sale_journal_entry.add_transaction_entry(revenue);
        sale_journal_entry.add_transaction_entry(cost_of_sales);

        assert_eq!(
            sale_journal_entry.total_debit(),
            sale_journal_entry.total_credit()
        );
        assert!(sale_journal_entry.validate());
        assert_eq!(sale_journal_entry.number_of_transaction_entries(), 4);
    }

    #[test]
    fn test_ledger_creation() {}
}
