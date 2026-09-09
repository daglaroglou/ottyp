use comfy_table::Table;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn print_accounts_table(accounts: Vec<(String, String, usize)>) {
    let mut table = Table::new();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    table.set_header(vec!["Account Name", "TOTP Code", "Valid For"]);

    for (name, code, step) in accounts {
        let step_u64 = if step == 0 { 30 } else { step as u64 };
        let valid_for = step_u64 - (now % step_u64);
        let time_str = format!("{}s", valid_for);
        table.add_row(vec![name, code, time_str]);
    }

    println!("{table}");
}
