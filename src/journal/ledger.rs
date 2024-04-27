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


///
/// `JournalEntry` structure that holds a set of related `TransactionEntries`.
/// The sum of the `Credit` entries must equal the `Debit` entries.
/// 
#[derive(Debug)]
struct JournalEntry {
    id: usize,
    transaction_entries: Vec<Rc<TransactionEntry>>,
    date_of_entry: DateTime<Utc>,
    description: String,
}

impl JournalEntry {
    fn new(id: usize, date_of_entry: DateTime<Utc>, description: &str) -> Self {
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