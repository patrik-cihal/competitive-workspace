//{"name":"New Year Chaos","group":"HackerRank - Algorithms - Constructive Algorithms - New Year Chaos - Prepare - Algorithms - Constructive Algorithms","url":"https://www.hackerrank.com/challenges/new-year-chaos/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"STDIN       Function\n-----       --------\n2           t = 2\n5           n = 5\n2 1 5 3 4   q = [2, 1, 5, 3, 4]\n5           n = 5\n2 5 1 3 4   q = [2, 5, 1, 3, 4]\n","output":"3\nToo chaotic\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NewYearChaos"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::collections::segment_tree::SegmentTree;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<usize>(n).iter().map(|x| x-1).collect::<Vec<usize>>();
    for i in 0..v.len() {
        if v[i] > i && v[i]-i > 2 {
            out_line!("Too chaotic");
            return;
        }
    }

    let mut st = SegmentTree::new(n+1, |a, b| a+b);
    for i in 0..n+1 {
        st.update(i, 0);
    }

    let mut answer = 0;
    for i in 0..v.len() {
        st.update(v[i], 1);
        let sum = st.query(v[i]+1..n+1).unwrap();
        answer += sum;
    }
    out_line!(answer);
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
