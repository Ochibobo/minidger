pub mod accounting_tree;
pub mod balance_sheet;
pub mod cashflow_statement;
pub mod income_statement;
pub mod ledger;

// let acc: Account = Account::new("Cash", asset);asset

// let cash_entry = TransactionEntry::new(acc, 
//     400.0, EntryType::Credit, Utc::now(),
//     "Electricity expense");
    
// let expense_acc: Account = Account::new("Utilities Expenses", expense);
// let expense_entry: TransactionEntry = TransactionEntry::new(expense_acc, 400.0, EntryType::Debit, Utc::now(),
//  "Electricity expense");

// let mut journal_entry: JournalEntry = JournalEntry::new(1, Utc::now(), "We paid for electricity");
// journal_entry.add_transaction_entry(cash_entry);
// journal_entry.add_transaction_entry(expense_entry);

// println!("{:?}", journal_entry);

// let is_journal_valid = journal_entry.validate();

// println!("{:?}", is_journal_valid);

// let mut ledger: Ledger = Ledger::new(1);
// ledger.add_journal_entry(journal_entry);

// let entry = ledger.get_journal_entry_by_id(1);
// let entry = entry.unwrap();

// println!("{:?}", entry);


// #[derive(Debug)]
// struct Account {
//     name: String,
//     account_tag: Rc<AccountTagNode>,
// }

// impl Account {
//     fn new(name: &str, account_tag: Rc<AccountTagNode>) -> Self {
//         Account {
//             name: name.to_owned(), account_tag
//         }
//     }

//     // Get the account name
//     pub fn name(&self) -> &str {
//         &self.name
//     }

//     pub fn account_type(&self) -> &AccountTagNode{
//         self.account_tag.as_ref()
//     }

//     pub fn set_name(&mut self, name: String){
//         self.name = name
//     }

//     pub fn set_account_type(&mut self, account_tag: Rc<AccountTagNode>){
//         self.account_tag = account_tag
//     }

//     ///
//     /// Get the `PrimaryAccountType` of this account
//     /// 
//     fn primary_account(&self) -> &Option<PrimaryAccountType> {
//         let mut parent_tag= self.account_tag.as_ref();

//         while parent_tag.level() != 1 {
//             let parent_tag = parent_tag.parent();
//         }
        
//         return &None;
//     }
// }
