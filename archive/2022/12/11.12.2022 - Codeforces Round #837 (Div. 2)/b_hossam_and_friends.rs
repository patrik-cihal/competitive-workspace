//{"name":"B. Hossam and Friends","group":"Codeforces - Codeforces Round #837 (Div. 2)","url":"https://codeforces.com/contest/1771/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3 2\n1 3\n2 3\n4 2\n1 2\n2 3\n","output":"4\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BHossamAndFriends"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut v = vec![n; n];
    for i in 0..m {
        let mut a = input.read::<usize>() - 1;
        let mut b = input.read::<usize>() - 1;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        v[a] = v[a].min(b);
    }
    for i in (0..n - 1).rev() {
        v[i] = v[i].min(v[i + 1]);
    }
    let mut answer = 0;
    for i in 0..n {
        answer += v[i] - i;
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
