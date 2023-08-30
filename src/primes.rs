pub struct PrimeGenerator {
    current: u32,
}

impl Iterator for PrimeGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current -= 1;
        self.current |= 1;
        loop {
            self.current += 2;
            if is_prime(self.current) {
                break;
            }
        }

        Some(result)
    }
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

pub fn enum_primes(start: u32) -> PrimeGenerator {
    let start = start.max(2);
    if start == 2 {
        return PrimeGenerator { current: 2 };
    }
    // Make start odd
    let start = start | 1;
    PrimeGenerator { current: start }
}

pub fn factorise(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut n = n;
    for prime in enum_primes(0) {
        if n <= 1 {
            break;
        }
        while n % prime == 0 {
            result.push(prime);
            n /= prime;
        }
    }

    result
}
