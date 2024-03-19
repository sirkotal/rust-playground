mod ex11;
use crate::ex11::Planet;

fn main() {
    let seconds = 1000000000;
    let duration = ex11::Duration::from(seconds);
    let output = ex11::Earth::years_during(&duration);
    println!("{}", output);
}