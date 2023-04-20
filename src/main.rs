use chrono::{NaiveDateTime, Duration};

mod cli;
mod constants;
mod renderer;

fn main() {
    let args = cli::parse_args();

    println!("{}", renderer::render_header(&args.name, &args.description));

    let mut from_date = args.start_date.clone();
    let mut to_date: NaiveDateTime = from_date + Duration::days(args.length.into());
    let mut user_id = 0;
    let max_user_id = args.users.len() - 1;

    while to_date < args.end_date {
        if user_id > max_user_id {
            user_id = 0;
        }
        let user = &args.users[user_id];
        println!("{}", renderer::render_user_schedule(&user, from_date, to_date));
        user_id += 1;
        from_date = from_date + Duration::days(args.length.into());
        to_date = to_date + Duration::days(args.length.into());
    }
}
