//{"name":"B. Rebellion","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n8\n0 0 1 1 1 1 1 1\n5\n1 0 0 1 1\n2\n1 0\n11\n1 1 0 0 1 0 0 1 1 1 0\n","output":"0\n1\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRebellion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let a = input.read_vec::<i32>(n);

    let mut i = 0;
    let mut j = a.len()-1;
    let mut result = 0;
    while i < j {
        if a[i] != 1 {
            i += 1;
        }
        else if a[j] != 0 {
            j -= 1;
        }
        else {
            result += 1;
            i += 1;
            j -= 1;
        }
    }
    out_line!(result);
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
