use crate::journal::ledger::Ledger;
use chrono::{DateTime, Utc};
///
/// `Balance Sheet` structure
/// This is basically a structured view of the ledger
///
pub struct BalanceSheet {
    id: usize,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    ledger: Ledger,
}

// BalanceSheet implementation
impl BalanceSheet {}
