// Submitted: `Rust 1.89.0 (2024)`
use std::{
    cmp::{Reverse, max, min},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

struct Group {
    idx_from: usize,
    idx_to: usize,
    vmax: i32,
    vmin: i32,
}

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let t: usize = inputs.next();

    for _ in 0..t {
        let n: usize = inputs.next();
        let mut a: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            a.push(inputs.next());
        }
        let mut q = VecDeque::<Group>::new();
        let mut gmin = a[n - 1];
        let mut gmax = a[n - 1];
        let mut gto = n - 1;
        for i in (0..(n - 1)).rev() {
            if a[i] <= gmin {
                // new range
                q.push_front(Group {
                    idx_from: i + 1,
                    idx_to: gto,
                    vmax: gmax,
                    vmin: gmin,
                });
                gto = i;
                gmin = a[i];
                gmax = a[i];
            } else if a[i] > gmax {
                // try merge
                gmax = a[i];
                while q.front().is_some_and(|g| g.vmin < gmax) {
                    let fr = q.pop_front().unwrap();
                    gto = fr.idx_to;
                    gmin = min(gmin, fr.vmin);
                    gmax = max(gmax, fr.vmax);
                }
            }
        }
        q.push_front(Group {
            idx_from: 0,
            idx_to: gto,
            vmax: gmax,
            vmin: gmin,
        });
        while let Some(g) = q.pop_front() {
            for i in g.idx_from..=g.idx_to {
                if i != n - 1 {
                    print!("{} ", g.vmax);
                } else {
                    println!("{}", g.vmax);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn main() {
    run();
}