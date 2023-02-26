//{"name":"C. Number Game","group":"Codeforces - Educational Codeforces Round 138 (Rated for Div. 2)","url":"https://codeforces.com/contest/1749/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 1 2\n4\n4 4 4 4\n1\n1\n5\n1 3 2 1 1\n","output":"2\n0\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNumberGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a = input.read_vec::<u32>(n);
    a.sort();
    a.reverse();

    let mut l = 0;
    let mut r = n;
    while l<r {
        let k = l+(r-l)/2+1;
        let mut j = 0;
        let mut i = 0;
        while i<k {
            let val = (k-i) as u32;
            while j<n-k+1 && a[j] > val {
                j += 1;
            }
            if j == n-k+1 {
                break;
            }
            j += 1;
            i += 1;
        }
        if i != k {
            r = k-1;
        }
        else {
            l = k;
        }
    }
    out_line!(l);
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
