use time::PrimitiveDateTime as DateTime;
use time::OffsetDateTime;
use time::Duration;

mod ex03;

fn main() {
    let now = OffsetDateTime::now_utc();
    let start = DateTime::new(now.date(), now.time());

    println!("{:?}", now);
    println!("{:?}", start);
    println!("{:?}", ex03::after(start));
}