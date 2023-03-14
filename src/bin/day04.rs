fn main() {
    // suppose given range is: 10000-20000
    //
    // set `start` to the first number higher or equal to start of given range where
    // the digits never decrease. 
    let start = 11111; // not real input

    // set `end` to the end of the given range
    let end = 20000; // not real input

    let mut n = start;
    let mut result1 = 0;
    let mut result2 = 0;
    while n <= end {
        if has_repeated_digit(n / 10, n % 10) {
            result1 += 1;
        }
        if has_digit_pair(n / 10, n % 10, 1) {
            result2 += 1;
        }
        n = increase(n);
    }

    println!("{}", result1);
    println!("{}", result2);
}

fn increase(n: usize) -> usize {
    if n == 9 {
        return 11;
    }
    if n < 10 || n % 10 != 9 {
        return n + 1;
    }

    let head = increase(n / 10);
    let last = head % 10;

    head * 10 + last
}

fn has_repeated_digit(n: usize, previous: usize) -> bool {
    if n == 0 {
        return false;
    }

    let last = n % 10;
    if last == previous {
        return true;
    }

    has_repeated_digit(n / 10, last)
}

fn has_digit_pair(n: usize, last: usize, count: usize) -> bool {
    if n < 10 {
        return (n == last && count == 1) || (n != last && count == 2);
    }

    if n % 10 == last {
        return has_digit_pair(n / 10, last, count + 1);
    }

    if count == 2 {
        return true;
    }

    has_digit_pair(n / 10, n % 10, 1)
}
