//{"name":"saving-leaves","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"saving-leaves"}}}

use std::collections::VecDeque;
use std::thread;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

#[derive(Clone)]
enum DPState {
    NotCalculated,
    Calculated { value: i64, keep: bool },
    Impossible,
}

fn go(a: &Vec<(i64, i64)>, limits: &Vec<i64>, k: usize, i: usize, mut j: usize, dp: &mut Vec<Vec<DPState>>) -> i64 {
    j += a[i].1 as usize; 
    match dp[i][j] {
        DPState::Calculated {value, ..} => {
            value
        }
        DPState::NotCalculated => {
            if j > limits[i] as usize {
                dp[i][j] = DPState::Impossible;
                return -1;
            }
            let cand1 = go(a, limits, k, i+1, j, dp);
            let cand2 = if j > 0 {a[i].0 + go(a, limits, k, i+1, j-1, dp)} else {-1}; 
            if cand2 > cand1 {
                dp[i][j] = DPState::Calculated { value: cand2, keep: true };
                cand2
            }
            else {
                dp[i][j] = DPState::Calculated { value: cand1, keep: false };
                cand1
            }
        },
        DPState::Impossible => {
            -1
        },
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();


    let mut a = vec![(0, 0); n];
    let mut limits = vec![0; n+1];

    let mut previous = vec![];

    for i in 0..n {
        a[i].0 = input.read();
        let ceil: i64 = input.read();
        if ceil == 1 {
            previous.push(i);
        }
        if i < k {
            a[0].1 += ceil;
        }
        else {
            a[i-k].1 += ceil;
        }
        limits[(i+k+1).min(n)] -= ceil;
    }

    let mut offset = 0;
    for i in 0..n {
        offset += a[i].1;
        offset += limits[i];
        limits[i] = offset;
    }


    let mut dp = vec![vec![DPState::NotCalculated; 5*k+1]; n];
    dp[n-1][0] = DPState::Calculated { value: 0, keep: false };
    dp[n-1][1] = DPState::Calculated { value: a[n-1].0, keep: true };
    dp[n-1][2..].iter_mut().for_each(|x| *x = DPState::Impossible);


    thread::Builder::new().stack_size(1024*1024*1024).spawn(move || {
        go(&a, &limits, k, 0, 0, &mut dp);
        let mut j = 0;
        let mut keepers = vec![0; n];
        for i in 0..n {
            j += a[i].1 as usize;
            if let DPState::Calculated { keep, .. } = dp[i][j] {
                if keep {
                    keepers[i] = 1;
                    j -= 1;
                }
            }
            else {
                panic!("WUTH");
            }
        }

        if let DPState::Calculated {value, ..} = dp[0][a[0].1 as usize] {
            out_line!(value);
        }
        else {
            panic!("WUT");
        }

        let mut j = 0;
        for i in 0..n {
            if keepers[i] == 1 {
                out_line!(previous[j]+1, i+1);
                j += 1;
            }
        }


    }).unwrap().join().unwrap();

}


pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
