//{"name":"C. Monoblock","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5 5\n1 2 3 4 5\n3 2\n4 2\n3 1\n2 1\n2 2\n","output":"29\n23\n35\n25\n35\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMonoblock"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut v = input.read_vec::<usize>(n);

    let mut ans = n*(n+1)/2;

    for i in 1..n {
        if v[i] != v[i-1] {
            ans += i*(n-i);
        }
    }

    for i in 0..m {
        let j: usize = input.read::<usize>()-1;
        let x = input.read();

        if j>0 && v[j] != v[j-1] {
            ans -= j*(n-j);
        }
        if j<n-1 && v[j] != v[j+1] {
            ans -= (j+1)*(n-j-1);
        }

        v[j] = x;

        if j>0 && v[j] != v[j-1] {
            ans += j*(n-j);
        }
        if j<n-1 && v[j] != v[j+1] {
            ans += (j+1)*(n-j-1);
        }

        out_line!(ans);
    }

}

pub(crate) fn run(mut input: Input) -> bool {
    let t = 1;
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
