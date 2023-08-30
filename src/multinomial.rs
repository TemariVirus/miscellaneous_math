use std::collections::HashMap;

pub fn binomial(n: u32) -> Vec<i64> {
    let mut coefs: Vec<i64> = vec![0; (n + 1) as usize];
    let mut term = 1;
    for i in 0..=n / 2 {
        coefs[i as usize] = term;
        coefs[(n - i) as usize] = term;
        term *= (n - i) as i64;
        term /= (i + 1) as i64;
    }
    coefs
}

pub fn expand_binomial(a: i64, b: i64, n: u32) -> Vec<i64> {
    let mut coefs = binomial(n);
    let mut term = a.pow(n);
    for item in &mut coefs {
        *item *= term;
        term /= a;
        term *= b;
    }
    coefs
}

pub fn multinomial(k: u32, n: u32) -> Vec<i64> {
    match k {
        0 => vec![],
        1 => vec![1],
        2 => binomial(n),
        3.. => multinomial_higher_dim(k, n, &mut HashMap::new()),
    }
}

fn multinomial_higher_dim(k: u32, n: u32, seen: &mut HashMap<(u32, u32), Vec<i64>>) -> Vec<i64> {
    if k == 2 {
        return binomial(n);
    }
    if seen.contains_key(&(k, n)) {
        return seen.get(&(k, n)).unwrap().clone();
    }

    let row = binomial(n);
    (0..=n)
        .flat_map(|i| {
            let mut hyperplane = multinomial_higher_dim(k - 1, i, seen);
            for item in &mut hyperplane {
                *item *= row[i as usize];
            }
            hyperplane
        })
        .collect::<Vec<i64>>()
}

pub fn expand_multinomial(coefs: &[i64], n: u32) -> Vec<i64> {
    let mut result = multinomial(coefs.len() as u32, n);
    expand_multinomial_helper(coefs, n, 1, &mut result, 0);
    result
}

fn expand_multinomial_helper(
    coefs: &[i64],
    m: u32,
    mut mul: i64,
    result: &mut [i64],
    mut idx: usize,
) -> usize {
    mul *= coefs[0].pow(m);
    if m == 0 || coefs.len() == 1 {
        result[idx] *= mul;
        return idx + 1;
    }
    for i in 0..=m {
        idx = expand_multinomial_helper(&coefs[1..], i, mul, result, idx);
        mul /= coefs[0];
    }
    idx
}

pub fn print_pascals_simplex(k: u32, n: u32) {
    print_pascals_simplex_recursive(k + 1, n, 1, n);
}

fn print_pascals_simplex_recursive(k: u32, n: u32, mul: i64, start_n: u32) {
    if k == 2 {
        print!("{:>width$}", "", width = (start_n - n) as usize);
        for x in binomial(n) {
            print!("{} ", x * mul);
        }
        return;
    }

    let row = binomial(n);
    for i in 0..=n {
        print_pascals_simplex_recursive(k - 1, i, mul * row[i as usize], start_n);
        println!();
    }
}
