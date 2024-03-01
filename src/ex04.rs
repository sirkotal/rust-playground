pub fn isPrime(n: u32) -> bool {
    if (n < 2) {
        return false;
    }
    let mut d = 2;

    while (d < ((n/2)+1)) {
        if (n % d == 0) {
            return false;
        }
        d += 1;
    }

    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut start = 3;
    let mut prime = 2;

    while (count != n) {
        if (isPrime(start)) {
            prime = start;
            count += 1;
        }
        start += 1;
    }

    return prime;
}