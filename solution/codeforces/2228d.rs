// Submitted: `Rust 2024`
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::RandomState,
};

//! TODO: Insert lib/rust/io.rs

#[derive(Clone)]
struct Pt {
    x: usize,
    y: usize,
}

fn run() {
    let mut io = IO4PS::new();
    io.read_all();
    let mut inputs = io.tokenize();

    let t: usize = inputs.next();

    for _ in 0..t {
        let n: usize = inputs.next();
        let mut pts_xs = Vec::with_capacity(n);
        let mut pts_ymax = Vec::with_capacity(n);
        let mut pts_ymin = Vec::with_capacity(n);
        for _ in 0..n {
            let ix: usize = inputs.next();
            let iy: usize = inputs.next();
            let pt = Pt { x: ix, y: iy };
            pts_xs.push(pt.clone());
            pts_ymax.push(pt.clone());
            pts_ymin.push(pt);
        }
        pts_xs.sort_by_key(|p| p.x);
        pts_ymin.sort_by_key(|p| p.y);
        pts_ymax.sort_by_key(|p| Reverse(p.y));

        let mut cy = vec![0; n + 1];
        let mut sy = vec![0; n + 1];
        for pt in pts_ymin.iter() {
            cy[pt.y] = 1;
        }
        sy[0] = 0;
        for i in 1..=n {
            sy[i] = sy[i - 1] + cy[i];
        }

        let mut cx = 0;
        let mut lmaxo = None;
        let mut lmino = None;
        let mut rmax_i = 0;
        let mut rmin_i = 0;
        let mut xsp = 0;
        let mut ans: u64 = 0;
        // line is at x=cx+0.5
        while cx < n {
            cx += 1;
            let mut flag = false;
            while xsp < n && pts_xs[xsp].x <= cx {
                if lmaxo.is_none_or(|v| pts_xs[xsp].y > v) {
                    lmaxo = Some(pts_xs[xsp].y);
                }
                if lmino.is_none_or(|v| pts_xs[xsp].y < v) {
                    lmino = Some(pts_xs[xsp].y);
                }
                xsp += 1;
                flag = true;
            }
            while rmax_i < n && pts_ymax[rmax_i].x <= cx {
                rmax_i += 1;
            }
            while rmin_i < n && pts_ymin[rmin_i].x <= cx {
                rmin_i += 1;
            }
            if !flag {
                continue;
            }
            if lmaxo.is_none() || lmino.is_none() || rmax_i == n || rmin_i == n {
                continue;
            }
            let lmax = lmaxo.unwrap();
            let lmin = lmino.unwrap();
            let rmax = pts_ymax[rmax_i].y;
            let rmin = pts_ymin[rmin_i].y;
            if lmax <= rmin || lmin >= rmax {
                continue;
            }
            let yleft = std::cmp::max(lmin, rmin);
            let yright = std::cmp::min(lmax, rmax);
            // println!(
            //     "x = {} + 0.5, left: {} .. {}, right: {} .. {} => cnt {}",
            //     cx,
            //     lmin,
            //     lmax,
            //     rmin,
            //     rmax,
            //     sy[yright - 1] - sy[yleft] + 1
            // );
            ans += sy[yright - 1] - sy[yleft] + 1
        }

        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn main() {
    run();
}
