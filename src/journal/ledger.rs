use crate::journal::accounting_tree::AccountNode;
use std::rc::Rc;
use chrono::{DateTime, Utc};

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
    account: Rc<AccountNode>,
    amount: f64,
    entry_type: EntryType,
    date_of_entry: DateTime<Utc>,
    description: String,
}

impl TransactionEntry {
    pub fn new(account: Rc<AccountNode>, amount: f64, entry_type: EntryType, 
        date_of_entry: DateTime<Utc>, description: &str) -> Self {
        TransactionEntry {
            account, amount, entry_type, date_of_entry, description: description.to_owned(),
        }
    }

    pub fn account(&self) -> Rc<AccountNode> {
        self.account.clone()
    }


    pub fn set_account(&mut self, account:Rc<AccountNode>) {
        self.account = account
    }

    pub fn amount(&self) -> &f64 {
        &self.amount
    }

    pub fn set_amount(&mut self, amount: f64){
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

    pub fn set_date_of_entry(&mut self, date_of_entry: DateTime<Utc>){
        self.date_of_entry = date_of_entry
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str){
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
        let transaction_entries: Vec<Rc<TransactionEntry>>= Vec::new();
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

    pub fn add_transaction_entry(&mut self, transaction_entry: Rc<TransactionEntry>){
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

    pub fn transaction_entries(&self) -> &Vec<Rc<TransactionEntry>> {
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
pub struct Ledger {
    id: usize,
    journal_entries: Vec<JournalEntry>,
}

impl Ledger {
    pub fn new(id: usize) -> Self {
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


#[cfg(test)]
mod test {

    ///
    /// Test the creation of a transaction entry
    /// 
    #[test]
    fn test_transaction_entry_creation() {

    }
}