use chrono::{NaiveDateTime};
use crate::constants::DATE_FORMAT;


pub fn render_header(name: &str, description: &str) -> String {
    let mut out: String = format!("---
# This file was generated using https://github.com/juandspy/ic-updater

$schema: /app-sre/schedule-1.yml

labels: {{}}

name: {}", name);
    if description != "" {
        out = format!("{}\ndescription: {}", &out, description);
    }
    out = format!("{}\nschedule:", &out);
    out
}

pub fn render_user_schedule(user: &str, from_date: NaiveDateTime, to_date: NaiveDateTime) -> String {
    let from_date = from_date.format(DATE_FORMAT).to_string();
    let to_date = to_date.format(DATE_FORMAT).to_string();
    format!(
        "- start: '{from_date}'
  end: '{to_date}'
  users:
  - $ref: {user}")
}
