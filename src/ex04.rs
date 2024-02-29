pub fn isPrime(n: u32) -> bool {
    let mut divs = 1;
    for i in 1..((n/2)+1) {
        if (n % i == 0) {
            divs += 1;
        }
    }

    if (divs == 2) {
        return true;
    }
    return false;
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut start = 2;
    let mut prime = 0;

    while (count != n) {
        if (isPrime(start)) {
            prime = start;
            count += 1;
        }
        start += 1;
    }

    return prime;
}