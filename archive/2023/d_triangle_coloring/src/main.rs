//{"name":"D. Triangle Coloring","group":"Codeforces - Educational Codeforces Round 143 (Rated for Div. 2)","url":"https://codeforces.com/contest/1795/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"12\n1 3 3 7 8 5 2 2 2 2 4 2\n","output":"36\n"},{"input":"6\n4 2 6 6 6 4\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTriangleColoring"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::math::nck_mod_p;


fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let P = 998244353;

    let mut a: Vec<u64> = input.read_vec(n);

    let mut mp = 1;


    for i in (2..n).step_by(3) {
        let mut v = vec![a[i], a[i - 1], a[i - 2]];
        v.sort();

        if v[0] & v[1] == v[2] {
            mp *= 3;
        }
        else if v[0] == v[1] {
            mp *= 2;
        }
        mp %= P;
    }


    let mut ans = nck_mod_p(n as u64/3, n as u64/6, P) * mp % P;

    out_line!(ans);


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
