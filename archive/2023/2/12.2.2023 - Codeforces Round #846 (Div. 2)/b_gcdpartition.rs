//{"name":"B. GCD Partition","group":"Codeforces - Codeforces Round #846 (Div. 2)","url":"https://codeforces.com/contest/1780/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4\n2 2 1 3\n2\n1 2\n3\n1 4 5\n6\n1 2 1 1 1 3\n10\n12 30 37 88 12 78 89 17 2 12\n6\n7 7 7 7 7 7\n","output":"4\n1\n5\n3\n1\n21\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGCDPartition"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    
    let a = input.read_vec::<i64>(n);

    let mut sum = a.iter().sum::<i64>();
    let mut sum2 = 0;

    let mut answer = 0;

    for i in 0..n-1 {
        sum -= a[i];
        sum2 += a[i];
        answer = answer.max(gcd(sum, sum2));
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
