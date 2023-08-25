use crate::primes::PrimeGenerator;

// Inspired by youtu.be/JY0_ApbZYkQ
pub fn u32_to_bracket_dot(n: u32) -> String {
    if n == 0 {
        return String::from(".");
    }

    let mut n = n;
    let mut result = String::from("(");
    for prime in PrimeGenerator::new(0) {
        if n == 1 {
            break;
        }

        let mut count = 0;
        while n % prime == 0 {
            count += 1;
            n /= prime;
        }
        result.push_str(u32_to_bracket_dot(count).as_str());
    }

    result.push(')');
    result
}

pub fn bracket_dot_to_u32(s: &str) -> Result<u32, &str> {
    if s == "." {
        return Ok(0);
    }
    if s == "()" {
        return Ok(1);
    }
    if !s.starts_with('(') || !s.ends_with(')') {
        return Err("Invalid bracket dot string");
    }
    let s = &s[1..s.len() - 1];

    let mut result = 1;
    let mut i = 0;
    for prime in PrimeGenerator::new(0) {
        if i >= s.len() {
            break;
        }
        if &s[i..i + 1] == "." {
            i += 1;
            continue;
        }
        if &s[i..i + 1] != "(" {
            return Err("Invalid bracket-dot string");
        }

        let sub = match get_until_close_bracket(&s[i..]) {
            Ok(sub) => sub,
            Err(()) => return Err("Invalid bracket-dot string"),
        };
        result *= prime.pow(bracket_dot_to_u32(sub)?);
        i += sub.len();
    }

    Ok(result)
}

fn get_until_close_bracket(s: &str) -> Result<&str, ()> {
    if &s[0..1] != "(" {
        return Err(());
    }

    let mut depth = 1;
    let mut i = 1;
    while depth > 0 {
        if i >= s.len() {
            return Err(());
        }
        match &s[i..i + 1] {
            "(" => depth += 1,
            ")" => depth -= 1,
            _ => (),
        }
        i += 1;
    }

    Ok(&s[0..i])
}
