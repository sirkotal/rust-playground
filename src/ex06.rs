pub fn square_of_sum(n: u32) -> u32 {
    let mut res = 0;
    for x in 1..(n+1) {
        res += x;
    }

    return u32::pow(res, 2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut res = 0;
    for x in 1..(n+1) {
        res += u32::pow(x, 2);
    }

    return res;
}

pub fn difference(n: u32) -> u32 {
    return (square_of_sum(n) - sum_of_squares(n));
}

pub fn square_of_sum_alt(n: u32) -> u32 {
    let mut res = (n * (n + 1)) / 2;

    return u32::pow(res, 2);
}

pub fn sum_of_squares_alt(n: u32) -> u32 {
    let mut res = (n * (n + 1) * ((2*n) + 1)) / 6;

    return res;
}

pub fn difference_alt(n: u32) -> u32 {
    return (square_of_sum_alt(n) - sum_of_squares_alt(n));
}