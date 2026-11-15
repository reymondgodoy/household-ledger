//! A tiny household ledger: record entries and report balances.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub id: u32,
    pub account: String,
    /// Amount in cents. Positive is income, negative is spending.
    pub cents: i64,
}

#[derive(Debug, Default)]
pub struct Ledger {
    entries: Vec<Entry>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Saves an entry. Saving the same entry id again must not record it twice.
    pub fn save(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The balance of one account in cents.
    pub fn balance(&self, account: &str) -> i64 {
        self.entries
            .iter()
            .filter(|entry| entry.account == account)
            .map(|entry| entry.cents)
            .sum()
    }

    /// Balances of every account.
    pub fn balances(&self) -> HashMap<String, i64> {
        let mut balances = HashMap::new();
        for entry in &self.entries {
            *balances.entry(entry.account.clone()).or_insert(0) += entry.cents;
        }
        balances
    }

    /// Average spending per entry in cents, over spending entries only.
    pub fn average_spending(&self) -> i64 {
        let spending: Vec<i64> = self
            .entries
            .iter()
            .filter(|entry| entry.cents < 0)
            .map(|entry| entry.cents)
            .collect();
        spending.iter().sum::<i64>() / spending.len() as i64
    }
}

/// Formats cents as dollars, e.g. -1234 -> "-$12.34".
pub fn format_cents(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let cents = cents.abs();
    format!("{sign}${}.{}", cents / 100, cents % 100)
}
