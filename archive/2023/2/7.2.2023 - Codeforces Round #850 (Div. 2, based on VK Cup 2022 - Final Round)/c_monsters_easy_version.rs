//{"name":"C. Monsters (easy version)","group":"Codeforces - Codeforces Round #850 (Div. 2, based on VK Cup 2022 - Final Round)","url":"https://codeforces.com/contest/1786/problem/C","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3\n3 1 2\n6\n4 1 5 4 1 1\n","output":"0\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMonstersEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a = input.read_vec::<u64>(n);

    a.sort();

    let mut cur = 1;
    let mut ans = 0;

    for a in a {
        if a >= cur {
            ans += a - cur;
            cur += 1;
        }
    }

    out_line!(ans);
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
