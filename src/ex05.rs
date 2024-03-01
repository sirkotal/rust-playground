pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num as u64;
    let mut digits = 0;
    let mut vector: Vec<u64> = Vec::new();
    let mut res = 0;

    while (n > 0) {
        let x = n % 10;
        vector.push(x);
        n /= 10;
        digits += 1;
    }

    for x in 0..vector.len() {
        res += u64::pow(vector[x], digits);
    }

    if (res == (num as u64)) {
        return true;
    }

    return false;
}