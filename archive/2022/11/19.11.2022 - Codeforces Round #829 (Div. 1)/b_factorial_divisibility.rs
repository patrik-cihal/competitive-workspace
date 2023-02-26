//{"name":"B. Factorial Divisibility","group":"Codeforces - Codeforces Round #829 (Div. 1)","url":"https://codeforces.com/problemset/problem/1753/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6 4\n3 2 2 2 3 3\n","output":"Yes\n"},{"input":"8 3\n3 2 2 2 2 2 1 1\n","output":"Yes\n"},{"input":"7 8\n7 7 7 7 7 7 7\n","output":"No\n"},{"input":"10 5\n4 3 2 1 4 3 2 4 3 4\n","output":"No\n"},{"input":"2 500000\n499999 499999\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFactorialDivisibility"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let x = input.read();
    let a = input.read_vec::<usize>(n);

    let mut count = vec![0; x];

    for val in a {
        count[val-1] += 1;
    }

    let mut answer = true;
    for i in 0..x-1 {
        if count[i] % (i+2) != 0 {
            answer = false;
            break;
        }
        else {
            count[i+1] += count[i]/(i+2);
        }
    }

    out_line!(if answer {"Yes"} else {"No"});


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
