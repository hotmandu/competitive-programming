// Submitted: `Rust 2024`
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
};

//! TODO: Insert lib/rust/io.rs

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let n: usize = inputs.next();

    let mut g = HashMap::new();
    let mut full = HashSet::new();
    let mut conn = HashSet::new();
    let mut dead = false;

    'outer: for _ in 0..n {
        let str = inputs.next_raw();
        let mut prv = '1';
        for (i, ch) in str.chars().enumerate() {
            full.insert(ch);
            if i > 0 {
                conn.insert(ch);
                if let Some(ach) = g.get(&prv)
                    && *ach != ch
                {
                    dead = true;
                    // println!("dead on {}: {} -> {}?", i, prv, ch);
                    break 'outer;
                }
                g.insert(prv, ch);
            }
            prv = ch;
        }
    }

    if !dead {
        let mut rmcnt = 0;
        let mut ents = BinaryHeap::from_iter(full.difference(&conn).map(|x| std::cmp::Reverse(x)));
        let mut ans: String = String::with_capacity(full.len());
        let mut put = HashSet::new();
        'outer: while !ents.is_empty() {
            let mut cur = ents.pop().map(|x| x.0);
            while let Some(chr) = cur {
                ans.push(*chr);
                if !put.insert(*chr) {
                    dead = true;
                    break 'outer;
                }
                rmcnt += 1;
                cur = g.get(chr);
            }
        }
        if dead {
            println!("NO");
        } else if rmcnt == full.len() {
            println!("{}", ans);
        } else {
            println!("NO");
        }
    } else {
        println!("NO");
    }
}

#[allow(dead_code)]
fn main() {
    run();
}
