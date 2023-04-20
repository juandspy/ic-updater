use clap::Parser;
use chrono::{format::ParseError, NaiveDateTime, Duration};
use crate::constants::DATE_FORMAT;


/// A tool to generate AppSRE schedules.
#[derive(Parser, Debug)]
pub struct Args {
    /// Name of yaml schema. F.e: MY_TEAM-ic-schedule
    #[arg(short, long)]
    pub name: String,

    /// Description of the yaml schema. F.e: MY_TEAM Interrupt Catcher schedule
    #[arg(short, long, default_value = "")]
    pub description: String,

    /// List of users. F.e: user-a,user-b,user-c
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    pub users: Vec<String>,

    /// Call duty length (in days)
    #[arg(short, long, default_value_t = 7)]
    pub length: u8,

    /// Starting date (YYYY-MM-DD hh:mm)
    #[arg(short, long)]
    #[arg(value_parser = parse_date)]
    pub start_date: NaiveDateTime,

    /// Ending date (YYYY-MM-DD hh:mm)
    #[arg(short, long)]
    #[arg(value_parser = parse_date)]
    pub end_date: NaiveDateTime,
}

fn parse_date(arg: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(arg, DATE_FORMAT)
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    validate_args(&args);
    args
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