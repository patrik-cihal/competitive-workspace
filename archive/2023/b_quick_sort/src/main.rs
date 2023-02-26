//{"name":"B. Quick Sort","group":"Codeforces - Codeforces Round #842 (Div. 2)","url":"https://codeforces.com/contest/1768/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 2\n1 2 3\n3 1\n3 1 2\n4 2\n1 3 2 4\n4 2\n2 3 1 4\n","output":"0\n1\n1\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BQuickSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    
    let k: usize = input.read();

    let p: Vec<usize> = input.read_vec(n);

    let mut last = 0;
    let mut count = 0;
    for i in 0..n {
        if p[i] == last+1 {
            last += 1;
            count += 1;
        }
    }

    out_line!((n-count+k-1)/k);
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
