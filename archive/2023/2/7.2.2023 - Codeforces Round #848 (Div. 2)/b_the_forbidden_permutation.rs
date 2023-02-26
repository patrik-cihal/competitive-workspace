//{"name":"B. The Forbidden Permutation","group":"Codeforces - Codeforces Round #848 (Div. 2)","url":"https://codeforces.com/contest/1778/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 2 2\n1 2 3 4\n1 3\n5 2 4\n5 4 3 2 1\n5 2\n5 3 3\n3 4 1 5 2\n3 1 2\n2 2 1\n1 2\n2 1\n6 2 4\n1 2 3 4 5 6\n2 5\n","output":"1\n3\n2\n0\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTheForbiddenPermutation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let d: usize = input.read();

    let p = input.read_vec::<usize>(n).into_iter().map(|x| x-1).collect::<Vec<usize>>();
    let a: Vec<usize> = input.read_vec::<usize>(m).into_iter().map(|x| x-1).collect::<Vec<usize>>();

    let mut result = usize::MAX;

    let mut mi = vec![0; n];
    for i in 0..p.len() {
        mi[p[i]] = i;
    }

    for i in 1..a.len() {
        let mn = a[i].min(a[i-1]);
        let mx = a[i].max(a[i-1]);

        if mi[a[i]] < mi[a[i-1]] {
            result = 0;
        }
        else {
            let offs = mi[a[i]]-mi[a[i-1]];
            if offs > d {
                result = 0;
            }
            else {
                result = result.min(offs);
                if d<n-1 { // or n
                    result = result.min(d+1-offs);
                }
            }
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
