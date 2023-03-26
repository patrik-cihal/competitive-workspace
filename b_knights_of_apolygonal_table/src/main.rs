//{"name":"B. Knights of a Polygonal Table","group":"Codeforces - Codeforces Round 488 by NEAR (Div. 2)","url":"https://codeforces.com/contest/994/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4 2\n4 5 9 7\n1 2 11 33\n","output":"1 3 46 36\n"},{"input":"5 1\n1 2 3 4 5\n1 2 3 4 5\n","output":"1 3 5 7 9\n"},{"input":"1 0\n2\n3\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BKnightsOfAPolygonalTable"}}}

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k): (usize, usize) = input.read();

    let p = input.read_vec::<usize>(n);
    let mut c = input.read_vec::<usize>(n);

    let mut p = p.into_iter().enumerate().map(|(i, x)| (x, i)).collect::<Vec<_>>();

    p.sort();

    let mut bh = BinaryHeap::new();

    let mut sum = 0;

    let mut answer = vec![0; n];

    for i in 0..p.len() {
        sum += c[p[i].1];
        answer[p[i].1] = sum;
        bh.push(Reverse(c[p[i].1]));
        if bh.len() > k {
            sum -= bh.pop().unwrap().0;
        }
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
