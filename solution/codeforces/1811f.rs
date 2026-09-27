// Submitted: `Rust 1.89.0 (2024)`
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let t: usize = inputs.next();
    let mut g: Vec<Vec<usize>> = vec![];
    'outer: for _ in 0..t {
        let n: usize = inputs.next();
        let m: usize = inputs.next();
        g = vec![Vec::new(); n];
        let mut indeg = vec![0; n];
        for _ in 0..m {
            let (u, v): (usize, usize) = (inputs.next::<usize>() - 1, inputs.next::<usize>() - 1);
            g[u].push(v);
            g[v].push(u);
            indeg[u] += 1;
            indeg[v] += 1;
        }
        let mut k = 0;
        for i in 0..n {
            if i * i > n {
                break;
            }
            if i * i == n {
                k = i;
                break;
            }
        }
        if k <= 2 {
            println!("NO");
            continue 'outer;
        }
        let mut cnt_fours = 0;
        let mut fours = vec![false; n];
        for i in 0..n {
            if indeg[i] == 4 {
                cnt_fours += 1;
                fours[i] = true;
            } else if indeg[i] != 2 {
                println!("NO");
                continue 'outer;
            }
        }
        if cnt_fours != k {
            println!("NO");
            continue 'outer;
        }
        // check flowers
        for i in 0..n {
            if fours[i] {
                // exact two fours connected
                let mut fours_conn = 0;
                let mut twos = 0;
                for v in &g[i] {
                    if fours[*v] {
                        fours_conn += 1;
                    } else {
                        twos += 1;
                        if twos >= 2 {
                            continue;
                        }
                        let mut prev = i;
                        let mut curr = *v;
                        let mut petal_len = 1;
                        loop {
                            let next = if g[curr][0] == prev {
                                g[curr][1]
                            } else {
                                g[curr][0]
                            };
                            if next == i {
                                if petal_len != k - 1 {
                                    println!("NO");
                                    continue 'outer;
                                }
                                break;
                            } else {
                                if fours[next] {
                                    println!("NO");
                                    continue 'outer;
                                }
                                petal_len += 1;
                                prev = curr;
                                curr = next;
                            }
                        }
                    }
                }
                if fours_conn != 2 {
                    println!("NO");
                    continue 'outer;
                }
                if twos != 2 {
                    println!("NO");
                    continue 'outer;
                }
            }
        }
        // count four groups
        let mut vs = vec![false; n];
        let mut fours_found = false;
        for i in 0..n {
            if fours[i] {
                if fours_found && !vs[i] {
                    println!("NO");
                    continue 'outer;
                }
                fours_found = true;
                vs[i] = true;
                let mut prev = i;
                let mut curr = i;
                for pn in &g[curr] {
                    if fours[*pn] {
                        curr = *pn;
                        break;
                    }
                }
                while curr != i {
                    vs[curr] = true;
                    for pn in &g[curr] {
                        if fours[*pn] && prev != *pn {
                            prev = curr;
                            curr = *pn;
                            break;
                        }
                    }
                }
            }
        }
        println!("YES");
    }
}

#[allow(dead_code)]
fn main() {
    run();
}
