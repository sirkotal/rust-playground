pub fn reverse(input: &str) -> String {
    let mut rev = String::new();
    for ch in input.chars() {
        rev.insert(0, ch);
    }

    return rev;
}

fn main() {
    println!("{}", reverse("hello"));
}
