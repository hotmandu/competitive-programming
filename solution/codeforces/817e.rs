// Submitted: `Rust 1.89.0 (2024)`
use std::{
    cmp::{Reverse, max, min},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

struct SparseSegtree {
    stats: u32,
    left: Option<Box<SparseSegtree>>,
    right: Option<Box<SparseSegtree>>,
}

impl SparseSegtree {
    fn new() -> Self {
        Self {
            stats: 0,
            left: None,
            right: None,
        }
    }

    /// value는 현재 2 ** now_dep 레벨에서는 현재 노드의 의미에 포함됨.
    fn put(&mut self, now_dep: u32, value: u32) {
        self.stats += 1;
        if now_dep != 0 {
            let is_right = (value & 2u32.pow(now_dep - 1)) > 0;
            if is_right {
                if let Some(rnode) = &mut self.right {
                    rnode.put(now_dep - 1, value);
                } else {
                    let mut rnode = Box::new(SparseSegtree::new());
                    rnode.put(now_dep - 1, value);
                    self.right = Some(rnode);
                }
            } else {
                if let Some(lnode) = &mut self.left {
                    lnode.put(now_dep - 1, value);
                } else {
                    let mut lnode = Box::new(SparseSegtree::new());
                    lnode.put(now_dep - 1, value);
                    self.left = Some(lnode);
                }
            }
        }
    }

    /// value는 현재 2 ** now_dep 레벨에서는 현재 노드의 의미에 포함됨.
    fn delete(&mut self, now_dep: u32, value: u32) {
        self.stats -= 1;
        if now_dep != 0 {
            let is_right = (value & 2u32.pow(now_dep - 1)) > 0;
            if is_right {
                if self.right.as_ref().is_some_and(|node| node.stats == 1) {
                    let _ = self.right.take();
                } else {
                    self.right
                        .as_deref_mut()
                        .map(|v| v.delete(now_dep - 1, value));
                }
            } else {
                if self.left.as_ref().is_some_and(|node| node.stats == 1) {
                    let _ = self.left.take();
                } else {
                    self.left
                        .as_deref_mut()
                        .map(|v| v.delete(now_dep - 1, value));
                }
            }
        }
    }

    fn query(&self, now_dep: u32, value: u32, leadership: u32, now_value: u32) -> u32 {
        if now_dep == 0 {
            if now_value < leadership {
                self.stats
            } else {
                0
            }
        } else {
            let is_v_right = (value & 2u32.pow(now_dep - 1)) > 0;
            let is_ls_right = (leadership & 2u32.pow(now_dep - 1)) > 0;
            if is_ls_right {
                if is_v_right {
                    self.left.as_ref().map_or(0, |n| {
                        n.query(
                            now_dep - 1,
                            value,
                            leadership,
                            now_value + 2u32.pow(now_dep - 1),
                        )
                    }) + self.right.as_ref().map_or(0, |n| n.stats)
                } else {
                    self.right.as_ref().map_or(0, |n| {
                        n.query(
                            now_dep - 1,
                            value,
                            leadership,
                            now_value + 2u32.pow(now_dep - 1),
                        )
                    }) + self.left.as_ref().map_or(0, |n| n.stats)
                }
            } else {
                if is_v_right {
                    self.right
                        .as_ref()
                        .map_or(0, |n| n.query(now_dep - 1, value, leadership, now_value))
                } else {
                    self.left
                        .as_ref()
                        .map_or(0, |n| n.query(now_dep - 1, value, leadership, now_value))
                }
            }
        }
    }
}

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let t: usize = inputs.next();
    let mut st = SparseSegtree::new();

    for _ in 0..t {
        let q: i32 = inputs.next();
        if q == 1 {
            let p = inputs.next();
            st.put(27, p);
        } else if q == 2 {
            let p = inputs.next();
            st.delete(27, p);
        } else {
            let p = inputs.next();
            let l = inputs.next();
            println!("{}", st.query(27, p, l, 0));
        }
    }
}

#[allow(dead_code)]
fn main() {
    run();
}