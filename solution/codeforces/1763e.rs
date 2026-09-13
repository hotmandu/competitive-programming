// Submitted: `Rust 2024`
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let p: usize = inputs.next();

    let mut prod = vec![0u64; p + 1];
    let mut dp = vec![None; p + 1];
    dp[0] = Some(0);

    for cp in 1..=p {
        for i in 2usize..=632 {
            let v = i * (i - 1) / 2;
            if cp >= v {
                if let Some(dpv) = dp[cp - v] {
                    if let Some(now) = dp[cp] {
                        if now > dpv + i {
                            dp[cp] = Some(dpv + i);
                            prod[cp] = prod[cp - v] + (i * dpv) as u64;
                        } else if now == dpv + i {
                            if prod[cp] < prod[cp - v] + (i * dpv) as u64 {
                                prod[cp] = prod[cp - v] + (i * dpv) as u64;
                            }
                        }
                    } else {
                        dp[cp] = Some(dpv + i);
                        prod[cp] = prod[cp - v] + (i * dpv) as u64;
                    }
                }
            }
        }
    }
    println!("{} {}", dp[p].unwrap(), prod[p]);
}

#[allow(dead_code)]
fn main() {
    run();
}
