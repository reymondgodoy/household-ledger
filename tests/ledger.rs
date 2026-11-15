use ledger::{format_cents, Entry, Ledger};

fn entry(id: u32, account: &str, cents: i64) -> Entry {
    Entry { id, account: account.into(), cents }
}

#[test]
fn balances_sum_each_account() {
    let mut ledger = Ledger::new();
    ledger.save(entry(1, "food", -1200));
    ledger.save(entry(2, "salary", 50000));
    ledger.save(entry(3, "food", -800));
    assert_eq!(ledger.balance("food"), -2000);
    assert_eq!(ledger.balances()["salary"], 50000);
}

#[test]
fn saving_the_same_entry_twice_records_it_once() {
    let mut ledger = Ledger::new();
    ledger.save(entry(1, "food", -1200));
    ledger.save(entry(1, "food", -1200));
    assert_eq!(ledger.len(), 1);
}

#[test]
fn average_spending_of_an_empty_ledger_is_zero() {
    assert_eq!(Ledger::new().average_spending(), 0);
}

#[test]
fn cents_are_always_two_digits() {
    assert_eq!(format_cents(1205), "$12.05");
    assert_eq!(format_cents(-1234), "-$12.34");
}
