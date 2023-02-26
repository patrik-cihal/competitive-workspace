//{"name":"C. Different Differences","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5 9\n4 12\n3 3\n3 4\n4 4\n4 6\n8 11\n","output":"1 3 4 7 8\n2 4 7 12\n1 2 3\n1 3 4\n1 2 3 4\n2 4 5 6\n1 2 3 5 6 7 8 11\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDifferentDifferences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn test(m: usize, n: usize, k: usize) {

}

fn solve(input: &mut Input, _test_case: usize) {
    let k: usize = input.read();
    let n: usize = input.read();

    let mut v = vec![1; k];
    for i in 1..k {
        v[i] = v[i-1]+i;
    }

    let mut s = 0;

    for i in (0..k).rev() {
        if v[i]+(k-i-1) <= n {
            s = i;
            break;
        }
    }


    for i in s+1..k {
        v[i] = v[i-1]+1;
    }

    out_line!(v);
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
