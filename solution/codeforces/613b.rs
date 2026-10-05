// Submitted: `Rust 1.89.0 (2024)`
use std::{
    cmp::{Reverse, max, min},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Display, Write},
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

fn main() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let n: usize = inputs.next();
    let a: i64 = inputs.next();
    let cf: i64 = inputs.next();
    let cm: i64 = inputs.next();
    let m: i64 = inputs.next();

    let mut lv: Vec<(usize, i64)> = Vec::with_capacity(n);
    for i in 0..n {
        lv.push((i, inputs.next()));
    }
    let mut rv = vec![0; n];
    lv.sort_by_key(|v| v.1);
    for i in 0..n {
        rv[lv[i].0] = i;
    }
    // println!("{lv:?}");
    // println!("{rv:?}");
    let mut d: Vec<i64> = vec![0; n];
    d[0] = 0;
    for i in 1..n {
        d[i] = d[i - 1] + (lv[i].1 - lv[i - 1].1) * (i as i64);
    }
    // println!("{d:?}");
    let mut rptr = n; // [rptr, n) 인 구간을 전부 Full
    while rptr > 0 && lv[rptr - 1].1 == a {
        rptr -= 1;
    }
    // [rptr, n) 은 기본 full
    let mut best_sc = 0;
    let mut best_f = 0; // [f, n) 은 Full
    let mut best_i = 0; // [0, i] 는 lv[i]+oh 로 맞춤
    let mut best_oh = 0;
    let mut mfc = 0;

    // basic
    best_sc = (lv[0].1 * cm) + ((n - rptr) as i64 * cf);
    best_f = rptr;
    best_i = 0;
    // println!("begin {best_sc} = [0, {best_i}]&{best_oh} + [{best_f}, {n}) {mfc}");
    while mfc <= m {
        if rptr == 0 {
            best_sc = (a * cm) + (n as i64 * cf);
            best_f = 0;
            best_i = 0;
            break;
        }

        // inv: flatten on [0, bl - 1] always works.
        // 안되는 최소 index 찾기, rptr 이상은 안된다고 생각
        // [bl, br] 사이에 안되는 index가 있음
        let mut bl = 1;
        let mut br = rptr;
        // find opt i
        while bl < br {
            let mid = (bl + br) / 2;
            let cost = d[mid] + mfc;
            if mid >= rptr || cost > m {
                // [mid, br) 안됨.
                br = mid;
            } else {
                // [bl, mid] 까진 됨.
                bl = mid + 1;
            }
        }
        let flc = d[bl - 1];
        let lo = m - (mfc + flc);
        let opth = std::cmp::min(lo / bl as i64, a - lv[bl - 1].1);
        let sc = ((lv[bl - 1].1 + opth) * cm) + ((n - rptr) as i64 * cf);
        // println!(
        //     "on {rptr}: {sc} = [0, {}] -> {} (+{opth}) ++ [{rptr}, {n}) {mfc}, with cost = {}",
        //     bl - 1,
        //     lv[bl - 1].1 + opth,
        //     mfc + flc + (opth * bl as i64)
        // );
        if sc > best_sc {
            best_sc = sc;
            best_f = rptr;
            best_i = bl - 1;
            best_oh = opth;
        }

        rptr -= 1;
        mfc += a - lv[rptr].1;
    }
    println!("{}", best_sc);
    let mut tlv = vec![0; n];
    for i in 0..n {
        tlv[i] = if rv[i] >= best_f {
            a
        } else if rv[i] <= best_i {
            lv[best_i].1 + best_oh
        } else {
            lv[rv[i]].1
        };
    }
    println!("{}", Joiner::new(" ", &tlv));
}