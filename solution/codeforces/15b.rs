// Submitted: `Rust 1.89.0 (2024)`
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

fn solve(n: i64, m: i64, (xl, xr): (i64, i64), (y1, y2): (i64, i64)) -> i64 {
    if xr < xl {
        solve(n, m, (xr, xl), (y2, y1))
    } else if y1 > y2 {
        solve(n, m, (xl, xr), (m - y1 + 1, m - y2 + 1))
    } else {
        let xa = 1 + (xr - xl);
        let ya = 1 + (y2 - y1);
        // rect = (l, r, w, h) => [l, l + w], [r, r + h]
        let rect1 = (1, 1, (n - xa), (m - ya));
        let rect2 = (xa, ya, (n - xa), (m - ya));
        // println!("{:?}, {:?}", rect1, rect2);
        let ans = n * m - (2 * (rect1.2 + 1) * (rect1.3 + 1));
        if rect2.0 <= rect1.0 + rect1.2 && rect2.1 <= rect1.1 + rect1.3 {
            // println!(
            //     "overlap {}",
            //     ((rect1.0 + rect1.2 - rect2.0 + 1) * (rect1.1 + rect1.3 - rect2.1 + 1))
            // );
            ans + ((rect1.0 + rect1.2 - rect2.0 + 1) * (rect1.1 + rect1.3 - rect2.1 + 1))
        } else {
            ans
        }
    }
}

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let t: usize = inputs.next();
    for _ in 0..t {
        let n: i64 = inputs.next();
        let m: i64 = inputs.next();
        let (x1, y1): (i64, i64) = (inputs.next(), inputs.next());
        let (x2, y2): (i64, i64) = (inputs.next(), inputs.next());
        println!("{}", solve(n, m, (x1, x2), (y1, y2)));
    }
}

#[allow(dead_code)]
fn main() {
    run();
}
