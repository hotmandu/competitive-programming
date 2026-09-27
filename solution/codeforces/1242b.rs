// Submitted: `Rust 1.89.0 (2024)`
use std::{
    cmp::{Reverse, max, min},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let n: usize = inputs.next();
    let m: usize = inputs.next();

    let mut ur: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    // let mut one_edges: Vec<(usize, usize)> = Vec::with_capacity(m);
    for _ in 0..m {
        let u: usize = inputs.next::<usize>() - 1usize;
        let v: usize = inputs.next::<usize>() - 1usize;
        // one_edges.push((inputs.next(), inputs.next()));
        ur[u].insert(v);
        ur[v].insert(u);
    }
    let mut cgn = 0;
    let mut vs = vec![0; n];
    let mut unresolved: HashSet<usize, RandomState> = HashSet::from_iter(0..n);
    for i in 0..n {
        if vs[i] != 0 {
            continue;
        }
        cgn += 1;
        unresolved.remove(&i);
        vs[i] = cgn;
        let mut q = VecDeque::new();
        let mut pcs = ur[i].clone();

        for j in unresolved.difference(&ur[i]) {
            q.push_back(*j);
        }
        while let Some(k) = q.pop_front() {
            unresolved.remove(&k);
            vs[k] = cgn;

            for j in pcs.difference(&ur[k]) {
                q.push_back(*j);
            }
            pcs = pcs.intersection(&ur[k]).map(|v| *v).collect();
        }
    }
    println!("{}", cgn - 1);
}

#[allow(dead_code)]
fn main() {
    run();
}