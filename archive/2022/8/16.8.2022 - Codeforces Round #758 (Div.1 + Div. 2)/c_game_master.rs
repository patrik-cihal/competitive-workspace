//{"name":"C. Game Master","group":"Codeforces - Codeforces Round #758 (Div.1 + Div. 2)","url":"https://codeforces.com/contest/1608/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 2 3 4\n1 2 3 4\n4\n11 12 20 21\n44 22 11 30\n1\n1000000000\n1000000000\n","output":"0001\n1111\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGameMaster"}}}

use std::collections::BTreeMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a: Vec<i32> = input.read_vec(n);
    let mut b: Vec<i32> = input.read_vec(n);

    let mut g = vec![vec![]; n];

    // lets make a graph of vertices where vertex (u, v) represent that player u can win against player v
    // the result will be all elements we can reach by dfs from highest node of a

    // sort a and add edges for a
    let mut sa: Vec<(i32, usize)> = a.iter().enumerate().map(|(i, x)| (*x, i)).collect();
    sa.sort();
    for i in 1..n {
        g[sa[i-1].1].push(sa[i].1);
    }

    // sort b and add edges for b
    let mut sb: Vec<(i32, usize)> = b.iter().enumerate().map(|(i, x)| (*x, i)).collect();
    sb.sort();
    for i in 1..n {
        g[sb[i-1].1].push(sb[i].1);
    }

    // make dfs and keep track of visited elements
    let mut seen = vec![false; n];
    let mut stack = vec![sa[n-1].1];

    seen[stack[0]] = true;

    while let Some(cv) = stack.pop() {
        for &v in &g[cv] {
            if !seen[v] {
                seen[v] = true;
                stack.push(v);
            }
        }
    }

    // if seen[i] print 1
    for i in 0..n {
        if seen[i] {
            out!('1');
        }
        else {
            out!('0');
        }
    }
    out_line!();

    out_line!();
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
