use chrono::{NaiveDateTime, NaiveDate};

#[test]
fn test_render_user_schedule() {
    let from_date: NaiveDateTime = NaiveDate::from_ymd_opt(2022, 7, 28).unwrap().and_hms_opt(7, 0, 0).unwrap();
    let to_date: NaiveDateTime = NaiveDate::from_ymd_opt(2022, 8, 28).unwrap().and_hms_opt(14, 0, 0).unwrap();

    let want = "- start: '2022-07-28 07:00'
  end: '2022-08-28 14:00'
  users:
  - $ref: /teams/TEST_TEAM/users/USER_B.yml";
    let got = ic_updater::render_user_schedule(
        "/teams/TEST_TEAM/users/USER_B.yml",
        from_date, to_date);
    assert_eq!(&got, want);
}

#[test]
fn test_render_header() {
    let want = "---
# This file was generated using https://github.com/juandspy/ic-updater

$schema: /app-sre/schedule-1.yml

labels: {}

name: SCHEDULE-NAME
description: SCHEDULE_DESCRIPTION
schedule:";
    let got = ic_updater::render_header("SCHEDULE-NAME", "SCHEDULE_DESCRIPTION");
    assert_eq!(&got, want);
}

#[test]
fn test_render_header_empty_description() {
    let want = "---
# This file was generated using https://github.com/juandspy/ic-updater

$schema: /app-sre/schedule-1.yml

labels: {}

name: SCHEDULE-NAME
schedule:";
    let got = ic_updater::render_header("SCHEDULE-NAME", "");
    assert_eq!(&got, want);
}
