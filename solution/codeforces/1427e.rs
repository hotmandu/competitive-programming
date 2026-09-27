// Submitted: `Rust 1.89.0 (2024)`
use std::{
    cmp::{Reverse, max, min},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

fn bw(n: u64) -> u32 {
    if n == 0 { 0 } else { n.ilog2() + 1 }
}

fn iho(n: u64) -> u64 {
    if n == 0 { 0 } else { 2u64.pow(bw(n) - 1) }
}

#[derive(Clone, Copy, Debug)]
enum How {
    Init,
    Sum(u64, u64),
    Xor(u64, u64),
}

fn perform_sum(how: &mut HashMap<u64, How>, a: u64, b: u64) {
    if !how.contains_key(&(a + b)) {
        how.insert(a + b, How::Sum(a, b));
    }
}

fn perform_xor(how: &mut HashMap<u64, How>, a: u64, b: u64) {
    if !how.contains_key(&(a ^ b)) {
        how.insert(a ^ b, How::Xor(a, b));
    }
}

fn try_reduce(n: u64, mut how: Option<&mut HashMap<u64, How>>) -> u64 {
    let th = n.next_power_of_two();
    let mut m = n * n;
    if let Some(how) = &mut how {
        // make n * n
        let mut npow = n;
        let mut nf = 0b10;
        let mut mcand = n;
        for _ in 1..(bw(n)) {
            perform_sum(how, npow, npow);
            npow *= 2;
            if n & nf != 0 {
                perform_sum(how, mcand, npow);
                mcand += npow;
            }
            nf *= 2;
        }
    }
    while m >= th {
        let msb = iho(m) / iho(n);
        if let Some(how) = &mut how {
            let mut nmcand = n;
            while nmcand != n * msb {
                perform_sum(how, nmcand, nmcand);
                nmcand *= 2;
            }
            perform_xor(how, m, n * msb);
        }
        m = m ^ (n * msb);
    }
    m
}

fn try_reduce2(n: u64, mut how: Option<&mut HashMap<u64, How>>) -> u64 {
    let th = n.next_power_of_two();
    let mut m = n * (iho(n) + 1);
    if let Some(how) = &mut how {
        // make n * 1000..01
        let mut npow = n;
        while npow != n * iho(n) {
            perform_sum(how, npow, npow);
            npow *= 2;
        }
        perform_sum(how, n, npow);
    }
    while m >= th {
        let msb = iho(m) / iho(n);
        if let Some(how) = &mut how {
            let mut nmcand = n;
            while nmcand != n * msb {
                perform_sum(how, nmcand, nmcand);
                nmcand *= 2;
            }
            perform_xor(how, m, n * msb);
        }
        m = m ^ (n * msb);
    }
    m
}

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let n: u64 = inputs.next();

    let mut curr = n;
    let mut bb = HashSet::new();
    let mut how: HashMap<u64, How> = HashMap::new();
    bb.insert(n);
    how.insert(n, How::Init);

    // for ci in (3..=n).step_by(2) {
    //     println!("--- {}", ci);
    //     curr = ci;
    loop {
        // println!("curr = {}, {:b}", curr, curr);
        if curr == 1 {
            break;
        }
        if (curr + 1).is_power_of_two() {
            // x = 01(1a)1
            // x+x = 1(1a)10
            // y=(x+x)^x 1(0a)01
            // y+y = 1(0a)010
            // x+y = 1(0a)000
            // (y+y)^(x+y) = 10
            perform_sum(&mut how, curr, curr);
            perform_xor(&mut how, 2 * curr, curr);
            let y = (2 * curr) ^ curr;
            perform_sum(&mut how, y, y);
            perform_sum(&mut how, curr, y);
            perform_xor(&mut how, curr + y, y + y);
            // we have 2 now
            let mut cx = curr;
            let mut pt = 2;
            while cx > 1 {
                perform_xor(&mut how, cx, pt);
                perform_sum(&mut how, pt, pt);
                cx ^= pt;
                pt *= 2;
            }
            break;
        }
        let rq = try_reduce(curr, None);
        let q = rq ^ curr;
        if q == 0 {
            let rq2 = try_reduce2(curr, None);
            let q2 = rq2 ^ curr;
            if q2 == 0 || (rq2 + 1).is_power_of_two() {
                // curr -> rrtq (1s)
                // println!("catched {}: {:b} -> {:b}", curr, curr, rq2);
                try_reduce2(curr, Some(&mut how));
                curr = rq2;
            } else if bw(curr) > bw(q2) {
                // curr -> rtq
                try_reduce2(curr, Some(&mut how));
                perform_xor(&mut how, rq2, curr);
                let nx = iho(curr) / iho(q2) * q2;
                let mut qcand = q2;
                while qcand != nx {
                    perform_sum(&mut how, qcand, qcand);
                    qcand *= 2;
                }
                perform_xor(&mut how, curr, nx);
                // println!(
                //     "{}: {:b} ^ {:b} -> {:b} -> {:b}",
                //     curr,
                //     curr,
                //     rq2,
                //     q2,
                //     curr ^ nx
                // );
                curr = curr ^ nx;
            } else {
                // curr -> rrtq
                if bw(curr) > bw(rq2) {
                    // println!("b{}: {:b} -> {:b}", curr, curr, rq2);
                    try_reduce2(curr, Some(&mut how));
                    curr = rq2;
                } else {
                    panic!("{}: {:b} -> {:b}", curr, curr, rq2);
                }
            }
        } else if bw(curr) > bw(q) {
            // curr -> q -> reduce
            try_reduce(curr, Some(&mut how));
            perform_xor(&mut how, rq, curr);
            let nx = iho(curr) / iho(q) * q;
            let mut qcand = q;
            while qcand != nx {
                perform_sum(&mut how, qcand, qcand);
                qcand *= 2;
            }
            perform_xor(&mut how, curr, nx);
            // println!(
            //     "{}: {:b} ^ {:b} -> {:b} -> {:b}",
            //     curr,
            //     curr,
            //     rq,
            //     q,
            //     curr ^ nx
            // );
            curr = curr ^ nx;
        } else {
            // curr -> rq
            if bw(curr) > bw(rq) {
                // println!("a{}: {:b} -> {:b}", curr, curr, rq);
                try_reduce(curr, Some(&mut how));
                curr = rq;
            } else {
                panic!("{}: {:b} -> {:b}", curr, curr, rq);
            }
        }
    }
    // }

    let mut graph: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut indeg: HashMap<u64, u64> = HashMap::new();

    let mut mq = VecDeque::new();
    let mut bb = HashSet::new();

    mq.push_back(1);
    bb.insert(n);
    while let Some(i) = mq.pop_front() {
        // if !how.contains_key(&i) {
        //     panic!("how? {}, {:b}", i, i);
        // }
        let h = how[&i];
        if bb.contains(&i) {
            continue;
        }
        // println!("how {i} = {h:?}");
        bb.insert(i);
        indeg.insert(i, 0);
        match h {
            How::Sum(a, b) => {
                graph.entry(a).or_default().push(i);
                (*indeg.entry(i).or_default()) += 1;
                if a != b {
                    graph.entry(b).or_default().push(i);
                    (*indeg.entry(i).or_default()) += 1;
                }
                if !bb.contains(&a) {
                    mq.push_back(a);
                }
                if !bb.contains(&b) {
                    mq.push_back(b);
                }
            }
            How::Xor(a, b) => {
                graph.entry(a).or_default().push(i);
                (*indeg.entry(i).or_default()) += 1;
                if a != b {
                    graph.entry(b).or_default().push(i);
                    (*indeg.entry(i).or_default()) += 1;
                }
                if !bb.contains(&a) {
                    mq.push_back(a);
                }
                if !bb.contains(&b) {
                    mq.push_back(b);
                }
            }
            How::Init => {}
        }
    }

    mq.clear();
    mq.push_back(n);

    let mut printq = VecDeque::new();

    while let Some(i) = mq.pop_front() {
        // println!("ma {}", i);
        printq.push_back(how[&i]);
        if let Some(cvtx) = graph.get(&i) {
            for j in cvtx {
                indeg.entry(*j).and_modify(|v| *v -= 1);
                if indeg[j] == 0 {
                    mq.push_back(*j);
                }
            }
        }
    }

    println!("{}", printq.len() - 1);
    for h in printq.iter() {
        match h {
            How::Sum(a, b) => println!("{} + {}", a, b),
            How::Xor(a, b) => println!("{} ^ {}", a, b),
            How::Init => {}
        }
    }
}

#[allow(dead_code)]
fn main() {
    run();
}