#![allow(dead_code)]
mod math_utils;

use math_utils::*;

fn main() {}

fn facorial_related_driver() {
    for i in -10..=10 {
        let estimate = facorial_related_naive(i);
        let exact = facorial_related_exact(i);
        println!(
            "a = {:3}: estimate = {:>20.8}, exact = {:>20.8}, error = {:>20.8}",
            i,
            estimate,
            exact,
            (estimate - exact).abs()
        );
    }
}

// This calculates f(a) = sum_(n=a)^(∞) n(-a)^(2a-n)
fn facorial_related_naive(a: i32) -> f64 {
    if a == 1 {
        return -0.25; // Diverges, re-check correctness
    }

    let neg_a = -a as f64;
    let two_a = 2 * a;

    let mut n = a;
    let mut result = 0f64;
    loop {
        let term = (n as f64) * neg_a.powi(two_a - n);
        result += term;
        n += 1;

        if term.abs() < f64::EPSILON {
            break;
        }
    }
    result
}

// The above sum should be equal to f(a) = -((-a)^(a+1)[a(a+1)-1])/(a+1)^2 (proven by trial and error, but not rigourously)
fn facorial_related_exact(a: i32) -> f64 {
    -(-a as f64).powi(a + 1) * (a * (a + 1) - 1) as f64 / ((a + 1) as f64).powi(2)
}

fn count_n_simplexes_driver() {
    print!("  d/n");
    for i in 0..=7 {
        print!("{:4} ", i);
    }
    println!();
    for i in 0..=8 {
        print!("{:4} ", i);
        for j in 0..=7 {
            print!("{:4} ", count_n_simplexes(i, j));
        }
        println!();
    }
}

fn count_n_hypercubes_driver() {
    print!("  d/n");
    for i in 1..=7 {
        print!("{:4} ", i);
    }
    println!();
    for i in 1..=7 {
        print!("{:4} ", i);
        for j in 1..=7 {
            print!("{:4} ", count_n_hypercubes(i, j));
        }
        println!();
    }
}
