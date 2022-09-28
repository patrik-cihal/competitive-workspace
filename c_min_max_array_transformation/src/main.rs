//{"name":"C. Min-Max Array Transformation","group":"Codeforces - Educational Codeforces Round 134 (Rated for Div. 2)","url":"https://codeforces.com/contest/1721/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n2 3 5\n7 11 13\n1\n1000\n5000\n4\n1 2 3 4\n1 2 3 4\n4\n10 20 30 40\n22 33 33 55\n","output":"5 4 2\n11 10 8\n4000\n4000\n0 0 0 0\n0 0 0 0\n12 2 3 15\n23 13 3 15\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMinMaxArrayTransformation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let a = input.read_vec::<i32>(n);
    let b = input.read_vec::<i32>(n);

    let mut ans_low = vec![0; n];
    let mut j = 0;
    for i in 0..n {
        while b[j] < a[i] {
            j += 1;
        }
        ans_low[i] = b[j]-a[i];
    }

    let mut j = n-1;
    let mut ans_high = vec![b[n-1]-a[n-1]; n];
    for i in (0..n-1).rev() {
        if a[i+1] > b[i] {
            j = i;
        }
        ans_high[i] = b[j]-a[i];
    }

    out_line!(ans_low);
    out_line!(ans_high);
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
