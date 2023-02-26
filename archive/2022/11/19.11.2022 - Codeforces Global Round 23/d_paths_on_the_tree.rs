//{"name":"D. Paths on the Tree","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n5 4\n1 2 1 3\n6 2 1 5 7\n5 3\n1 2 1 3\n6 6 1 4 10\n","output":"54\n56\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPathsOnTheTree"}}}

use std::cmp::Ordering;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2, RecursiveFunction3};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();

    let mut g = vec![(vec![], k); n];
    for i in 1..n {
        let p = input.read::<usize>()-1;
        g[p].0.push(i);
    }

    // might cause stack overflow
    let mut fill_levels = RecursiveFunction2::new(|f, v: usize, l: usize| {
        g[v].1 = l;
       for child in &g[v].0 {
           f.call(*child, l/g[v].0.len());
       }
    });
    fill_levels.call(0, k);


    let c = input.read_vec::<i64>(n);
    let mut dp = vec![vec![-1; 2]; n];

    let mut go = RecursiveFunction2::new(|f, &v: &usize, b: usize| -> i64 {
        if dp[v][b] == -1 {
            let k = g[v].1+b;
            dp[v][b] = c[v]*k as i64;
            if !g[v].0.is_empty() {
                let mut candidates = vec![];
                for child in &g[v].0 {
                    let a = f.call(child, 0);
                    let b = f.call(child, 1);
                    candidates.push((b-a, b, child));
                }

                candidates.sort();
                candidates.reverse();
                let mut u = k%g[v].0.len();
                if u == 0 && b == 1 {
                    u = g[v].0.len();
                }
                for i in 0..g[v].0.len() {
                    if i<u {
                        dp[v][b] += candidates[i].1;
                    }
                    else {
                        dp[v][b] += candidates[i].1-candidates[i].0;
                    }
                }
            };
        }

        dp[v][b]
    });
    out_line!(go.call(&0, 0));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
