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

- Theink through the effect of:
  - Liabilities | Expenses | Dividends | Revenue
- 