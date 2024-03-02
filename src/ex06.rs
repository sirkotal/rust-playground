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
