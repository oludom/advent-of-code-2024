use std::time::SystemTime;
use cached::proc_macro::cached;

const NUM_EVALUATIONS: usize = 75;

#[cached]
fn expand(val: i64) -> Vec<i64> {
    if val == 0 {
        return vec![1]
    }
    let s = val.to_string();
    let l = s.len();
    if l != 0 && l % 2 == 0 {
        let lt = (l/2) as usize;
        let left = &s[..lt];
        let right = &s[lt..];

        return vec![left.parse().unwrap(), right.parse().unwrap()]
    }
    else {
        vec![2024*val]
    }
}

#[cached]
fn expand_rec(vals: Vec<i64>, it: usize) -> usize {
    let mut count = 0;
    for el in vals {
        let curr = expand(el);
        if it > 0 {
            let sub = expand_rec(curr, it-1);
            count += sub;
        }
        else {
            count += curr.len();
        }
    }
    count
}

fn timeit<F: Fn() -> T, T>(f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("function took {} ms", duration.as_millis());
    result
}

// result 272673043446478
fn solve() {
    let initial = vec![27, 10647, 103, 9, 0, 5524, 4594227, 902936];
    let mut count = 0;
    for stone in initial {
        println!("stone {stone}");
        
        count += expand_rec(vec![stone], NUM_EVALUATIONS-1)
    }
    println!("result: {count}");
}

fn main() {
    timeit(solve);
}
