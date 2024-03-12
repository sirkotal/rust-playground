mod ex10;

fn main() {
    println!("{}", ex10::Clock::new(23, 61).to_string());
    println!("{}", ex10::Clock::new(-25, 00).to_string()); // 23:00
    println!("{}", ex10::Clock::new(-25, -160).to_string()); // 20:20

    let clock = ex10::Clock::new(1, 1).add_minutes(3500);
    println!("{}", clock.to_string());

    let clock2 = ex10::Clock::new(00, 00).add_minutes(-1);
    println!("{}", clock2.to_string());

    let clock3 = ex10::Clock::new(1, -160).to_string(); // "22:20"
    println!("{}", clock3.to_string());
}