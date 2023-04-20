use chrono::{format::ParseError, NaiveDateTime, Duration};
use clap::Parser;

static DATE_FORMAT: &str = "%Y-%m-%d %H:%M";

/// A tool to generate AppSRE schedules.
#[derive(Parser, Debug)]
struct Args {
    /// Name of yaml schema. F.e: MY_TEAM-ic-schedule
    #[arg(short, long)]
    name: String,

    /// Description of the yaml schema. F.e: MY_TEAM Interrupt Catcher schedule
    #[arg(short, long, default_value = "")]
    description: String,

    /// List of users. F.e: user-a,user-b,user-c
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    users: Vec<String>,

    /// Call duty length (in days)
    #[arg(short, long, default_value_t = 7)]
    length: u8,

    /// Starting date (YYYY-MM-DD hh:mm)
    #[arg(short, long)]
    #[arg(value_parser = parse_date)]
    start_date: NaiveDateTime,

    /// Ending date (YYYY-MM-DD hh:mm)
    #[arg(short, long)]
    #[arg(value_parser = parse_date)]
    end_date: NaiveDateTime,
}

fn parse_date(arg: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(arg, DATE_FORMAT)
}

fn main() {
    let args = Args::parse();

    validate_args(&args);

    println!("{}", render_header(&args.name, &args.description));

    let mut from_date = args.start_date.clone();
    let mut to_date: NaiveDateTime = from_date + Duration::days(args.length.into());
    let mut user_id = 0;
    let max_user_id = args.users.len() - 1;

    while to_date < args.end_date {
        if user_id > max_user_id {
            user_id = 0;
        }
        let user = &args.users[user_id];
        println!("{}", render_user_schedule(&user, from_date, to_date));
        user_id += 1;
        from_date = from_date + Duration::days(args.length.into());
        to_date = to_date + Duration::days(args.length.into());
    }
}

fn exit_usage(msg: &str) {
    eprintln!("ERROR: {}", msg);
    std::process::exit(exitcode::USAGE);
}

fn validate_args(args: &Args) {
    if args.users.len() == 0 {
        // this cannot happen but just in case
        exit_usage("at least 1 user is needed");
    }

    if args.end_date < args.start_date {
        exit_usage(&format!("end date '{}' should be greater than date '{}'", args.end_date, args.start_date));
    }

    let min_end_date = args.start_date + Duration::days(args.length.into());
    if min_end_date > args.end_date {
        exit_usage(&format!("end date '{}' should be after start date + length of schedule, which is '{}'", args.end_date, min_end_date));
    }
}

fn render_header(name: &str, description: &str) -> String {
    let mut out: String = format!("---
# This file was generated using https://github.com/juandspy/ic-updater

$schema: /app-sre/schedule-1.yml

labels: {{}}

name: {}", name);
    if description != "" {
        out = format!("{}\ndescription: {}", &out, description);
    }
    out
}

fn render_user_schedule(user: &str, from_date: NaiveDateTime, to_date: NaiveDateTime) -> String {
    let from_date = from_date.format(DATE_FORMAT).to_string();
    let to_date = to_date.format(DATE_FORMAT).to_string();
    format!(
        "- start: '{from_date}'
  end: '{to_date}'
  users:
  - $ref: {user}")
}



#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate,};


    #[test]
    fn test_render_user_schedule() {
        let from_date: NaiveDateTime = NaiveDate::from_ymd_opt(2022, 7, 28).unwrap().and_hms_opt(7, 0, 0).unwrap();
        let to_date: NaiveDateTime = NaiveDate::from_ymd_opt(2022, 8, 28).unwrap().and_hms_opt(14, 0, 0).unwrap();

        let want = "- start: '2022-07-28 07:00'
  end: '2022-08-28 14:00'
  users:
  - $ref: /teams/TEST_TEAM/users/USER_B.yml";
        let got = render_user_schedule(
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
description: SCHEDULE_DESCRIPTION";
        let got = render_header("SCHEDULE-NAME", "SCHEDULE_DESCRIPTION");
        assert_eq!(&got, want);
    }

    #[test]
    fn test_render_header_empty_description() {
        let want = "---
# This file was generated using https://github.com/juandspy/ic-updater

$schema: /app-sre/schedule-1.yml

labels: {}

name: SCHEDULE-NAME";
        let got = render_header("SCHEDULE-NAME", "");
        assert_eq!(&got, want);
    }

}
