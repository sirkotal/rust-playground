pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    let mut p = 2;
    let mut res: Vec<u64> = Vec::new();

    while (p < ((n/2)+1)) {
        if (n % p == 0) {
            primes.push(p);
        }
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
