//{"name":"C. Sort Zero","group":"Codeforces - Codeforces Round #813 (Div. 2)","url":"https://codeforces.com/contest/1712/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n3 3 2\n4\n1 3 1 3\n5\n4 1 5 3 2\n4\n2 4 1 2\n1\n1\n","output":"1\n2\n4\n3\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSortZero"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v: Vec<usize> = input.read_vec(n);

    let mut last = 0;

    let mut seen = vec![false; n+1];

    let mut ans = 0;
    for i in 1..n {
        if seen[v[i]] {
            for j in last..i+1 {
                seen[v[j]] = true;
            }
            last = i+1;
        }
        else if  v[i]<v[i-1] {
            for j in last..i {
                seen[v[j]] = true;
            }
            last = i;
        }
    }

    let mut seen = vec![false; n+1];
    for i in 0..last {
        if !seen[v[i]] {
            ans += 1;
            seen[v[i]] = true;
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
