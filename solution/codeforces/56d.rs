// Submitted: `Rust 1.89.0 (2024)`
use std::{
    cmp::{Reverse, max, min},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

#[derive(Clone)]
enum How {
    Nothing,
    Replace,
    Delete,
    Insert,
}

fn main() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let s = inputs.next_raw().chars().collect::<Vec<char>>();
    let t = inputs.next_raw().chars().collect::<Vec<char>>();

    // d[i][j] = s[0..i] => t[0..j] 로 만드는데 필요한 최소 move, mv[i][j] 는 이때 선택한 move
    // d[i][j] = d[i][j-1] + 1 (Insert)
    //         = d[i-1][j] + 1 (Delete)
    //         = d[i-1][j-1] + 1 (Replace)
    //         = d[i-1][j-1] (그대로도 OK, only if s[i - 1] == j[i - 1])
    let mut d = vec![vec![i32::MAX - 1; 1001]; 1001];
    let mut mv = vec![vec![Option::<How>::None; 1001]; 1001];
    // Init
    d[0][0] = 0;
    mv[0][0] = Some(How::Nothing);

    for j in 0..=t.len() {
        for i in 0..=s.len() {
            if i == 0 && j == 0 {
                continue;
            }
            if j == 0 {
                d[i][j] = d[i - 1][j] + 1;
                mv[i][j] = Some(How::Delete);
            } else if i == 0 {
                d[i][j] = d[i][j - 1] + 1;
                mv[i][j] = Some(How::Insert);
            } else {
                if d[i][j] > d[i - 1][j] + 1 {
                    d[i][j] = d[i - 1][j] + 1;
                    mv[i][j] = Some(How::Delete);
                }
                if d[i][j] > d[i][j - 1] + 1 {
                    d[i][j] = d[i][j - 1] + 1;
                    mv[i][j] = Some(How::Insert);
                }
                if d[i][j] > d[i - 1][j - 1] + 1 {
                    d[i][j] = d[i - 1][j - 1] + 1;
                    mv[i][j] = Some(How::Replace);
                }
                if s[i - 1] == t[j - 1] && d[i][j] > d[i - 1][j - 1] {
                    d[i][j] = d[i - 1][j - 1];
                    mv[i][j] = Some(How::Nothing);
                }
            }
        }
    }

    let mut rv = VecDeque::new();
    let mut sptr = s.len();
    let mut tptr = t.len();
    while sptr != 0 || tptr != 0 {
        match &mv[sptr][tptr] {
            Some(How::Nothing) => {
                sptr -= 1;
                tptr -= 1;
            }
            Some(How::Delete) => {
                rv.push_back((How::Delete, tptr + 1, s[sptr - 1]));
                sptr -= 1;
            }
            Some(How::Insert) => {
                rv.push_back((How::Insert, tptr, t[tptr - 1]));
                tptr -= 1;
            }
            Some(How::Replace) => {
                rv.push_back((How::Replace, tptr, t[tptr - 1]));
                sptr -= 1;
                tptr -= 1;
            }
            None => panic!("Should not reachable"),
        }
    }
    println!("{}", rv.len());
    while let Some((v, p, c)) = rv.pop_back() {
        match v {
            How::Nothing => {}
            How::Replace => println!("REPLACE {} {}", p, c),
            How::Delete => println!("DELETE {}", p),
            How::Insert => println!("INSERT {} {}", p, c),
        }
    }
}