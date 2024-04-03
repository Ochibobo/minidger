### High Level Design Description Of MiniDger

Entities:
  - Journal Entry
  - Balance Sheet
  - Income Statement
  - CashFlow statement
  - Singlar Transactions
  - Database Views
  - Audit Trails
  - Database Views
  - Store:
    - Graph
    - Key-value 
    - Relational
    - Document
  - Time filtering
    - Indexing
  - DSL
    - Relation between accounts
    - Relation between financial statements
  - Account Types
  - Audit Trail


#### Journal Entry Design
- ID
- Date of entry
- Date of transaction
- Transactions
- Description

##### Transaction Design
- Account
  - Indent the name of the account that is being credited so that it is easier to see
- Amount
- Debit/Credit
- Description

##### Constraints
- Credits == Debits
- Balance Sheet rules
- Income Statement Rules
- Cash Flow rules

#### Bank Reconciliation
- Find match:
  - Between open invoices and deposits

- Think through account relations and hierarchies:
  - Assets
    - Current Asset
      - Cash

- Think through the effect of:
  - Liabilities | Expenses | Dividends | Revenue

- Accounting Cycle:
  - Identify Transaction
  - Prepare Journal Entry
    - Indent credit accounts for identification
  - Post to general ledger
  - Unadjusted Trial Balance
    - You list out all of your accounts and their closing balances
    - This will need a CLI output
    - Originally used to check that debits and credits are in balance
  - Post Adjusting Entries
    - Journal entries posted at the end of an accounting period to bring the books in line with the accrual method of accounting
    - Follow IFRS or GAAP
    - Accrual Method - recognize revenue as you earn it and record your expenses as you incur them
    - Cash accounting is not the same as accrual accounting
    - In cash accounting your recognize the revenue as you receive cash and record expenses as you pay it out
  - Adjusted Trial Balance
  - Create Financial Statements
  - Post closing entries
  - Repeat

Generally, there are 6 types of accounts:
- Assets
- Liabilities
- Equity
- Revenue
- Expenses 
- Dividends (Withdrawals)


##### Balance Sheet
- Has the company name
- Has the duration
- Has a currency symbol
- Each balance-sheet subgroup oughts to have a totals section.
  
##### Income Statement

##### Cash Flow Statement

##### Closing Entries

##### Accounting Equation
- DEALER
- Equity = Owner's Equity - Dividends + Retained Earnings
- Retained Earnings = Revenue - Expenses
- Equity = Owner's Equity - Dividends + Revenue - Expenses
- Assets = Liabilities + Owner's Equity - Dividends + Revenue - Expenses
- Assets + Dividends + Expenses = Liabilities + Owner's Equity + Revenue
- Economic benefit is the potential for an asset to contribute to the flow of an entity's cash
- There are sources and destinations
- Debits represent the flow of economic benefits to the destination
- Credits represent the flow of economic benefits from the source
- Debits and credits are opposite in banking:
  - Deposits are credited, which increases the account owner's balance
    - The bank debits their accounts to increase it but credits the amount owing to the account holder as it owns them.
    - Chequing accounts are liabilities.
  - Withdrawals are debited
  - From the bank's point of view, our chequing account is a liability, not an asset


##### T Accounts
- Is a visual representation of an account
- __Account__ - A place where we can record, sort & store all transactions that affect a related group of items
- __General Ledger__ - A place where a business stores a complete record of its financial transactions and accounts
- Debits(Dr) go on the left, Credits(Cr) go on the right.
- They allow one to spot errors in the general ledger


##### Account Tree
- Level 0 is the tree's root
- All account types are level 1 tags
- All other tags and terminal accounts occupy level 3 henceforth

- Account type not found, did you mean "accounts examples" instead
  - Provide indexed options to select which account they meant or a way to create a new one
  - Allow them to edit just the account entry made
    - Account entry is cached before persistence
  - This is applicable to journal entri


- Build the balance sheet tree beforehand
  - The shell tree, that is, then traverse accounts to fill it in.
  - Use Levels
  - Use PartialOrd to determine which subtree comes first etc

- Think of splitting parent trait
  - Add a child trait
    - When one sets a node's parent, add to the parent this node as a child
    - When one add's a node as the child of another:
      - Check if parent is already set.
      - If set, throw an error
      - If not, set the parent to this parent
      - Add method to set_parent
  - This ensures the terminal AccountNodes cannot be parents
  - Do I really need levels?
    - Think about traversal

- Account Type is an `Iterator` - No Need
- Read about Rc, RefCell, Box and Interior Mutability in Rust


###### std::cell
- Shareble mutable container
- RefCell & Mutex are here too
- Provide interior mutability
- You can replace, change and get a copy of a value inside a cell
  - That's why it's always safe to change it
  - No-one else has a pointer to it
- Does not represent Sync
  - Reference cannot be given to another thread
- Rc<T> only allows immutable borrows
- Only compatible with types that implement `Copy`

- Look into adding a `RefCell` to the relevant nodes.