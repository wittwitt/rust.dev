use chrono::{DateTime, Local, TimeZone};

use std::time::SystemTime;

fn main() {
    let now: DateTime<Local> = Local::now();
    let dt2: DateTime<Local> = Local.timestamp(0, 0);

    println!("{}", now);
    println!("{}", dt2);

    std::thread::sleep(std::time::Duration::from_secs(10));

    let c = Local::now() - now;
    println!("{}", c.num_milliseconds());

    let now = SystemTime::now();
    println!("{:?}", now);

    std::thread::sleep(std::time::Duration::from_secs(10));

    println!("{:?}", now.elapsed().unwrap());
}

mod t {
    use std::time::SystemTime;
    use time::Weekday::Wednesday;
    use time::{Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};

    fn f1() {
        let date = Date::from_iso_week_date(2022, 1, Wednesday).unwrap();
        let datetime = date.with_hms(13, 0, 55).unwrap();
        let datetime_off = datetime.assume_offset(UtcOffset::from_hms(1, 2, 3).unwrap());

        println!("{date}, {datetime}, {datetime_off}");
    }

    #[test]
    fn f2() {
        f1()
    }

    #[test]
    fn t2() {
        let now = SystemTime::now();
        println!("{:?}", now);
    }
}
