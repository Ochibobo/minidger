use std::fmt::Debug;

use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
enum ActionType {
    Increase,
    Decrease,
}

#[derive(Debug, PartialEq)]
struct AccountType {
    name: String,
    on_debit: ActionType,
    on_credit: ActionType,
}

impl AccountType {
    fn new(name: &str, on_debit: ActionType, on_credit: ActionType) -> Self {
        // Ascertain on_increase isn't the same as on_decrease
        if on_debit == on_credit {
            println!("Invalid account actions set. on_debit:ActionType = {:?} == on_credit:ActionType = {:?}!", on_debit, on_credit);
        }

        AccountType {
            name: name.to_owned(), on_debit, on_credit
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn on_debit(&self) -> &ActionType {
        &self.on_debit
    }

    pub fn on_credit(&self) -> &ActionType {
        &self.on_credit
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_action_type(&mut self, on_debit: ActionType, on_credit: ActionType) {
        if on_debit == on_credit {
            println!("Invalid account actions set. on_debit:ActionType = {:?} == on_credit:ActionType = {:?}!", on_debit, on_credit);
            return;
        }

        self.on_debit = on_debit;
        self.on_credit = on_credit;
    }
}

///
/// `AccountTreeNode` trait used to build and account's relational tree
/// Any node that's present on the account's tree is required to implement this trait
///
trait AccountTreeNode {
    // Used to retrieve the level of a node
    fn level(&self) -> usize;

    // Used to set the level of a node
    fn set_level(&mut self, level: usize);

    // Used to get the parent of a node
    fn parent(&self) -> &dyn AccountTreeNode;

    // Used to set the parent of a node
    fn set_parent(&mut self, parent: &dyn AccountTreeNode);
}

///
/// `AccountTag` structure used to define the category an account belongs to
///
struct AccountTagNode {
    level: usize,
    name: String,
    parent: Box<dyn AccountTreeNode>,
    children: Vec<Box<dyn AccountTreeNode>>,
}

impl Debug for AccountTagNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AccountTreeNode for AccountTagNode {
    fn level(&self) -> usize {
        return self.level;
    }

    fn set_level(&mut self, level: usize) {
        self.level = level;
    }

    fn parent(&self) -> &dyn AccountTreeNode {
        return self.parent.as_ref()
    }

    fn set_parent(&mut self, parent: &dyn AccountTreeNode) {
        // self.parent = Box::<dyn AccountTreeNode>::new(parent);
    }
}

impl AccountTagNode {
    fn new(level: usize, name: &str, parent: Box<dyn AccountTreeNode>) -> Self {
        let children = Vec::new();

        AccountTagNode{
            level, name: name.to_owned(), parent, children,
        }
    }

    ///
    /// Add a child to the `AccountTagNode`
    /// 
    fn add_child(&mut self, child: Box<dyn AccountTreeNode>) {
        self.children.push(child);
    }

    ///
    /// Get the children for this `AccountTagNode`
    /// 
    fn children(&self) -> &Vec<Box<dyn AccountTreeNode>> {
        return &self.children;
    }

    ///
    /// Get the name of the `AccountTagNode`
    /// 
    fn name(&self) -> &str {
        return self.name.as_str();
    }

    ///
    /// Set the name of the `AccountTagNode`
    /// 
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

///
/// Node representing an actual account on the `AccountTree`
struct AccountNode {
    level: usize,
    name: String,
    amount: f64,
    parent: Box<dyn AccountTreeNode>
}

impl Debug for AccountNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AccountTreeNode for AccountNode {
    fn level(&self) -> usize {
        return self.level
    }

    fn set_level(&mut self, level: usize) {
        self.level = level
    }

    fn parent(&self) -> &dyn AccountTreeNode {
        return self.parent.as_ref()
    }

    fn set_parent(&mut self, parent: &dyn AccountTreeNode) {
        todo!()
    }
}

impl AccountNode {
    fn new(level: usize, name: &str, amount: f64, parent: Box<dyn AccountTreeNode>) -> Self {
        AccountNode{
            level, name: name.to_owned(), amount, parent
        }
    }

    ///
    /// Used to get the name of the `AccountNode`
    /// 
    fn name(&self) -> &str {
        return &self.name;
    }

    ///
    /// Used to set the name of the `AccountNode`
    /// 
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    ///
    /// Used to set the amount in the `AccountNode`
    /// 
    fn set_amount(&mut self, amount: f64) {
        self.amount = amount
    }

    ///
    /// Used to get the amount in the `AccountNode`
    /// 
    fn amount(&self) -> f64 {
        return self.amount;
    }

    ///
    /// Used to get the parent of the `AccountNode`
    /// 
    fn parent(&self) -> &dyn AccountTreeNode {
        return self.parent.as_ref();
    }

    ///
    /// Used to set the parent of the `AccountNode`
    /// 
    fn set_parent(&mut self, parent: Box<dyn AccountTreeNode>) {
        self.parent = parent;
    }
}

#[derive(Debug, PartialEq)]
struct Account {
    name: String,
    account_type: AccountType,
}

impl Account {
    fn new(name: &str, account_type: AccountType) -> Self {
        Account {
            name: name.to_owned(), account_type
        }
    }

    // Get the account name
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn account_type(&self) -> &AccountType{
        &self.account_type
    }

    pub fn set_name(&mut self, name: String){
        self.name = name
    }

    pub fn set_account_type(&mut self, account_type: AccountType){
        self.account_type = account_type
    }
}

#[derive(Debug, PartialEq)]
enum EntryType {
    Credit,
    Debit,
}

///
/// `TransactionEntry` is a single row entry that makes up a JournalEntry
/// 
#[derive(Debug)]
struct TransactionEntry {
    account: Account,
    amount: f64,
    entry_type: EntryType,
    date_of_entry: DateTime<Utc>,
    description: String,
}

impl TransactionEntry {
    fn new(account: Account, amount: f64, entry_type: EntryType, 
        date_of_entry: DateTime<Utc>, description: &str) -> Self {
        TransactionEntry {
            account, amount, entry_type, date_of_entry, description: description.to_owned(),
        }
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn amount(&self) -> &f64 {
        &self.amount
    }

    pub fn entry_type(&self) -> &EntryType {
        &self.entry_type
    }

    pub fn date_of_entry(&self) -> &DateTime<Utc> {
        &self.date_of_entry
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug)]
struct JournalEntry {
    id: usize,
    transaction_entries: Vec<TransactionEntry>,
    date_of_entry: DateTime<Utc>,
    description: String,
}

impl JournalEntry {
    fn new(id: usize, date_of_entry: DateTime<Utc>, description: &str) -> Self {
        let transaction_entries: Vec<TransactionEntry>= Vec::new();
        JournalEntry {
            id, transaction_entries, date_of_entry, description: description.to_owned(),
        }
    }

    pub fn set_id(&mut self, id: usize){
        self.id = id
    }

    pub fn set_date_of_entry(&mut self, date_of_entry: DateTime<Utc>) {
        self.date_of_entry = date_of_entry
    }

    pub fn set_description(&mut self, description: String){
        self.description = description
    }

    pub fn add_transaction_entry(&mut self, transaction_entry: TransactionEntry){
        self.transaction_entries.push(transaction_entry);
    }

    pub fn id(&self) -> usize{
        self.id
    }

    pub fn date_of_entry(&self) -> DateTime<Utc> {
        self.date_of_entry
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn transaction_entries(&self) -> &Vec<TransactionEntry> {
        &self.transaction_entries
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

        return debits == credits
    }
}

///
/// `General Ledger` that comprises of a set of journal entries.
/// This is the structure that feeds into the `balance sheet`, the `income statement`
/// and the `statemement of cashflow`.
/// 
#[derive(Debug)]
struct Ledger {
    id: usize,
    journal_entries: Vec<JournalEntry>,
}

impl Ledger {
    fn new(id: usize) -> Self {
        Ledger{
            id, journal_entries: Vec::new(),
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
    /// Add a single journal entry
    /// 
    pub fn add_journal_entry(&mut self, journal_entry: JournalEntry){
        self.journal_entries.push(journal_entry);
    }

    ///
    /// Add multiple journal entries
    /// 
    pub fn add_journal_entries(&mut self, journal_entries: &mut Vec<JournalEntry>){
        self.journal_entries.append(journal_entries);
    }

    ///
    /// Replace all journal entries with the new one
    /// 
    pub fn set_journal_entries(&mut self, journal_entries: Vec<JournalEntry>){
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
       self.journal_entries.iter().filter(|j| j.date_of_entry == date_of_entry).into_iter().collect()
    }

    ///
    /// Get journal entries by date range (date of entry range)
    ///
    pub fn get_journal_entry_by_between(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Vec<&JournalEntry> {
        self.journal_entries.iter().filter(|j| j.date_of_entry >= start_date && j.date_of_entry <= end_date).into_iter().collect()
    } 
    
    ///
    /// Get a journal entries by description
    ///
    pub fn get_journal_entry_by_description(&self, description: &str) -> Vec<&JournalEntry> {
        self.journal_entries.iter().filter(|j| j.description.contains(description)).into_iter().collect()
    }

    ///
    /// Get the `number of journal entries` in the `ledger`
    /// 
    pub fn number_of_journal_entries(&self) -> usize {
        self.journal_entries.len()
    }
}

///
/// `Balance Sheet` structure
/// This is basically a structured view of the ledger
/// 
struct BalanceSheet {
    id: usize,
}

///
/// `Income Statement` structure
/// 
struct IncomeStatement {
    id: usize,
}


///
/// `Cash Flow` statement structure
/// 
struct CashFlowStatement {
    id: usize,
}

fn main() {
    let asset: AccountType = AccountType::new("Current Assets", ActionType::Increase, ActionType::Decrease);
    let expense: AccountType = AccountType::new("Expenses", ActionType::Increase, ActionType::Decrease);

    let acc: Account = Account::new("Cash", asset);

    let cash_entry = TransactionEntry::new(acc, 
        400.0, EntryType::Credit, Utc::now(),
        "Electricity expense");
        
    let expense_acc: Account = Account::new("Utilities Expenses", expense);
    let expense_entry: TransactionEntry = TransactionEntry::new(expense_acc, 400.0, EntryType::Debit, Utc::now(),
     "Electricity expense");
    
    let mut journal_entry: JournalEntry = JournalEntry::new(1, Utc::now(), "We paid for electricity");
    journal_entry.add_transaction_entry(cash_entry);
    journal_entry.add_transaction_entry(expense_entry);

    println!("{:?}", journal_entry);

    let is_journal_valid = journal_entry.validate();

    println!("{:?}", is_journal_valid);

    let mut ledger: Ledger = Ledger::new(1);
    ledger.add_journal_entry(journal_entry);
    
    let entry = ledger.get_journal_entry_by_id(1);
    let entry = entry.unwrap();

    println!("{:?}", entry);
}

