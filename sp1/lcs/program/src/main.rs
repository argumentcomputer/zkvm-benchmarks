#![no_main]
sp1_zkvm::entrypoint!(main);

use std::cmp::max;
use std::collections::VecDeque;

fn lcs_dyn(xs: &str, ys: &str) -> String {
    let xs: Vec<char> = xs.chars().collect();
    let ys: Vec<char> = ys.chars().collect();
    let (m, n) = (xs.len(), ys.len());

    let mut tab: Vec<VecDeque<i32>> = vec![VecDeque::from(vec![0; n + 1]); m + 1];

    for (i, &x) in xs.iter().enumerate() {
        let mut row = VecDeque::from(vec![0]);
        for (j, &y) in ys.iter().enumerate() {
            let val = if x == y {
                1 + tab[i][j]
            } else {
                max(tab[i][j + 1], row[j])
            };
            row.push_back(val);
        }
        tab[i + 1] = row;
    }

    construct(&xs, &ys, &tab)
}

fn construct(xs: &[char], ys: &[char], tab: &[VecDeque<i32>]) -> String {
    let mut result = Vec::new();
    let (mut i, mut j) = (xs.len(), ys.len());

    while i > 0 && j > 0 {
        if xs[i - 1] == ys[j - 1] {
            result.push(xs[i - 1]);
            i -= 1;
            j -= 1;
        } else if tab[i - 1][j] > tab[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.reverse();
    result.into_iter().collect()
}

pub fn main() {
    let input = sp1_zkvm::io::read::<(String, String)>();

    sp1_zkvm::io::commit(&input);

    let lcs = lcs_dyn(&input.0, &input.1);

    sp1_zkvm::io::commit(&lcs);
}
