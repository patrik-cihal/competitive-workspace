//{"name":"G2. Subsequence Addition (Hard Version)","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/G2","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1\n1\n1\n2\n5\n5 1 3 2 1\n5\n7 1 5 2 1\n3\n1 1 1\n5\n1 1 4 2 1\n","output":"YES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G2SubsequenceAdditionHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    
    let mut a = input.read_vec::<u64>(n);

    a.sort();

    let mut sum = 1;

    if a[0] != 1 {
        out_line!("NO");
        return;
    }

    for i in 1..n {
        if a[i] > sum {
            out_line!("NO");
            return;
        }
        sum += a[i];
    }

    out_line!("YES");
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
