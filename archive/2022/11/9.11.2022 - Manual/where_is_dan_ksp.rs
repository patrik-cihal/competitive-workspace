//{"name":"where_is_dan_ksp","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"where_is_dan_ksp"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();
    let k: usize = input.read();
    let x: usize = input.read();

    let mut p = input
        .read_vec::<usize>(m)
        .iter()
        .map(|x| x - 1)
        .collect::<Vec<usize>>();
    p.sort();

    let mut answer = 0;

    let dist = |p1: usize, p2: usize, n: usize| -> usize {
        if p1 >= p2 {
            p1 - p2
        } else {
            p1 + (n - p2)
        }
    };

    for i in 0..p.len() {

        let j = (i+m-x+1)%m;

        if dist(p[i], p[j], n) >= k {
            continue;
        }

        let d = dist(p[i], p[j], n);
        let lo = k - (d + 1);

        let b = (dist(p[j], p[(j + m - 1) % m], n) - 1).min(lo);
        let f = (dist(p[(i + 1) % m], p[i], n) - 1).min(lo);

        if d + b + f + 1 < k {
            continue;
        }

        let result = 1 + b + f - lo;
        // out_line!(p[i], p[j], ddresult);
        answer += result;
    }

    out_line!(answer);
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
