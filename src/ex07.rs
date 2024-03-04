pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);
    primes.push(3);
    let mut p = 1;
    let mut res: Vec<u64> = Vec::new();

    while (((6 * p) - 1) < n) {
        primes.push((6 * p) - 1);
        primes.push((6 * p) + 1);
        p += 1;
    }

    let mut i = 0;
    let mut num = n;

    while (num > 1) {
        if (num % primes[i] == 0) {
            res.push(primes[i]);
            num /= primes[i];
        }
        else {
            i += 1;
        }
    }

    return res;
}
