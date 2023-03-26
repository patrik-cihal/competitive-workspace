//{"name":"E. Tree Master","group":"Codeforces - Codeforces Round 858 (Div. 2)","url":"https://codeforces.com/contest/1806/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"6 2\n1 5 2 3 1 1\n1 2 3 3 2\n4 5\n6 6\n","output":"33\n27\n"},{"input":"14 8\n3 2 5 3 1 4 2 2 2 5 5 5 2 4\n1 2 3 1 1 4 7 3 3 1 5 3 8\n4 4\n4 10\n13 10\n3 12\n13 9\n3 12\n9 10\n11 5\n","output":"47\n53\n48\n36\n42\n36\n48\n14\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETreeMaster"}}}

use std::collections::{HashSet, HashMap};

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

const sn: usize = 320;

fn go(mut v: usize, mut u: usize, p: &Vec<usize>, d: usize, scores: &Vec<u64>, dp: &mut Vec<Vec<Option<u64>>>, depth_count: &Vec<usize>, depth_index: &Vec<usize>) -> u64 {
    if v > u {
        std::mem::swap(&mut u, &mut v);
    }
    if depth_count[d] <= sn {
        if let Some(result) = dp[v][depth_index[u]] {
            return result;
        }
    }
    let result = scores[v]*scores[u]+go(p[v-1], p[u-1], p, d-1, scores, dp, depth_count, depth_index);
    if depth_count[d] <= sn {
        dp[v][depth_index[u]] = Some(result);
    }
    result
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, q): (usize, usize) = input.read();

    let mut g = vec![vec![]; n];

    let scores = input.read_vec::<u64>(n);
    let p = input.read_vec::<usize>(n-1).into_iter().map(|x| x-1).collect::<Vec<usize>>();

    for i in 0..p.len() {
        g[p[i]].push(i+1);
    }

    let mut depth_count = vec![0; n];
    let mut stack = vec![(0, 0)];
    let mut depth = vec![0; n];
    let mut depth_index = vec![0; n];

    while let Some((v, d)) = stack.pop() {
        depth_index[v] = depth_count[d];
        depth_count[d] += 1;
        depth[v] = d;
        for u in &g[v] {
            stack.push((*u, d+1));
        }
    }

    let mut dp = vec![vec![None; sn]; n];
    dp[0][0] = Some(scores[0]*scores[0]);
    for _ in 0..q {
        let (x, y): (usize, usize) = input.read();

        out_line!(go(x-1, y-1, &p, depth[x-1], &scores, &mut dp, &depth_count, &depth_index));
    }
    
}

pub(crate) fn run(mut input: Input) -> bool {
    for i in 0usize..1 {
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
