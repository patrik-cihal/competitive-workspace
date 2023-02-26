//{"name":"B. Going to the Cinema","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, based on VK Cup 2022 - Elimination Round)","url":"https://codeforces.com/contest/1782/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 1\n7\n0 1 2 3 4 5 6\n8\n6 0 3 3 6 7 2 7\n5\n3 0 0 3 3\n","output":"2\n1\n3\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGoingToTheCinema"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a: Vec<usize> = input.read_vec(n);
    a.sort();

    let mut answer = 1;

    let mut count = 0;
    let mut higher = false;

    for i in 0..n {
        if a[i] <= count {
            if higher {
                answer += 1;
                higher = false;
            }
        }
        else {
            higher = true;
        }
        count += 1;
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
