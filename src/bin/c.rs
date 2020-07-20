#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let tmp_l = l % 2019;
    let tmp_r = r % 2019;
    if tmp_l >= tmp_r || tmp_l == 0 || tmp_r == 0 || r - l >= 2019 {
        println!("0");
    } else {
        let mut ans = std::usize::MAX;
        for i in tmp_l..=tmp_r {
            for j in i + 1..=tmp_r {
                ans = min(ans, (i * j) % 2019);
            }
        }
        println!("{}", ans);
    }
}
