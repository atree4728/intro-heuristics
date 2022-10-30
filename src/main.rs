use proconio::{fastout, input, marker::Usize1};

const M: usize = 26;

#[fastout]
fn main() {
    input! {
        d: usize,
        c: [i32; M],
        s: [[i32; M]; d],
        t: [Usize1; d]
    }
    let mut score = 0;
    let mut last = vec![0; M];
    for i in 0..d {
        score += s[i][t[i]];
        last[t[i]] = i + 1;
        for j in 0..M {
            score -= c[j] * (i as i32 + 1 - last[j] as i32);
        }
        println!("{}", score);
    }
}
