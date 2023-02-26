//{"name":"A. Flip Flop Sum","group":"Codeforces - Codeforces Round #848 (Div. 2)","url":"https://codeforces.com/contest/1778/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n-1 1 1 -1 -1\n5\n1 1 -1 -1 -1\n2\n1 1\n4\n1 -1 -1 1\n","output":"3\n3\n-2\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AFlipFlopSum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<i32>(n);

    let mut contains = false;
    let sum = v.iter().sum::<i32>();

    for i in 1..v.len() {
        if v[i] == v[i - 1] && v[i] == -1 {
            contains = true;
        }
    }

    if contains {
        out_line!(sum+4);
        return;
    }
    else if v.contains(&(-1)) {
        out_line!(sum);
    }
    else {
        out_line!(sum-4);
    }
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
