// Submitted: `Rust 2024`
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

/// Sort `[l, r)`
fn merge_sort(
    a: &mut Vec<u32>,
    l: usize,
    r: usize,
    dep: usize,
    dep_prog: &mut Vec<u64>,
    dep_inv: &mut Vec<u64>,
) {
    // println!("M {}\t..\t{}\tdep {}\t IN", l, r, dep);
    if l + 1 == r {
        return;
    }
    let mid = (l + r) / 2;
    merge_sort(a, l, mid, dep - 1, dep_prog, dep_inv);
    merge_sort(a, mid, r, dep - 1, dep_prog, dep_inv);
    // println!("M {}\t..\t{}\tdep {}\tmid={}", l, r, dep, mid);

    // merge
    let mut lptr = l;
    let mut rptr = mid;
    let mut progs = 0u64;
    let mut invs = 0u64;
    let mut b = Vec::with_capacity(r - l + 1);
    while (lptr < mid) || (rptr < r) {
        // println!("M {}\t..\t{}\tdep {}\t loop {}, {}", l, r, dep, lptr, rptr);
        if lptr < mid && (rptr == r || (rptr != r && a[lptr] < a[rptr])) {
            // pick left
            // println!("M {}\t..\t{}\tdep {}\t  pick left", l, r, dep);
            b.push(a[lptr]);
            progs += (r - rptr) as u64;
            lptr += 1;
        } else if rptr < r && (lptr == mid || (lptr != mid && a[lptr] > a[rptr])) {
            // pick right
            // println!("M {}\t..\t{}\tdep {}\t  pick right", l, r, dep);
            b.push(a[rptr]);
            invs += (mid - lptr) as u64;
            rptr += 1;
        } else {
            // same
            // println!("M {}\t..\t{}\tdep {}\t  same", l, r, dep);
            let mut lpicks = 0;
            let mut rpicks = 0;
            let s = a[lptr];
            while lptr < mid && a[lptr] == s {
                lpicks += 1;
                lptr += 1;
                b.push(s);
            }
            while rptr < r && a[rptr] == s {
                rpicks += 1;
                rptr += 1;
                b.push(s);
            }
            // println!(
            //     "M {}\t..\t{}\tdep {}\t s={}\t{}, {}",
            //     l, r, dep, s, lptr, rptr
            // );
            progs += lpicks * (r - rptr) as u64;
            invs += rpicks * (mid - lptr) as u64;
        }
    }
    for i in l..r {
        a[i] = b[i - l];
    }
    dep_prog[dep] += progs;
    dep_inv[dep] += invs;
}

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let n: usize = inputs.next();
    let mut a: Vec<u32> = Vec::new();
    for _ in 0..(2usize.pow(n as u32)) {
        a.push(inputs.next());
    }
    let mut dep_prog = vec![0u64; n + 1];
    let mut dep_inv = vec![0u64; n + 1];

    merge_sort(
        &mut a,
        0,
        2usize.pow(n as u32),
        n,
        &mut dep_prog,
        &mut dep_inv,
    );

    let m: usize = inputs.next();
    let mut qs: Vec<usize> = Vec::new();
    for _ in 0..m {
        qs.push(inputs.next());
    }
    let mut fl = vec![false; n + 1];
    for q in qs {
        for i in 0..=q {
            fl[i] = !fl[i];
        }
        let mut ans: u64 = 0;
        for i in 0..=n {
            if fl[i] {
                ans += dep_prog[i];
            } else {
                ans += dep_inv[i];
            }
        }
        io.writeln(ans);
    }
    io.flush();
}

#[allow(dead_code)]
fn main() {
    run();
}
