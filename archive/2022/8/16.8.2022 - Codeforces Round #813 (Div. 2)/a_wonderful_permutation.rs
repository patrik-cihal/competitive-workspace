//{"name":"A. Wonderful Permutation","group":"Codeforces - Codeforces Round #813 (Div. 2)","url":"https://codeforces.com/contest/1712/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 1\n2 3 1\n3 3\n1 2 3\n4 2\n3 4 1 2\n1 1\n1\n","output":"1\n0\n2\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AWonderfulPermutation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();

    let mut ans = 0;

    let v = input.read_vec::<usize>(n);

    for i in 0..k {
        if v[i] > k {
            ans += 1;
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
