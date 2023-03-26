//{"name":"B. Vaccination","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 3 5 3\n1 2 3 10 11 18\n6 4 0 0\n3 3 3 3 3 4\n9 10 2 2\n0 1 2 3 4 5 6 7 8\n3 10 3 6\n10 20 30\n5 5 4 4\n0 2 4 6 8\n","output":"2\n3\n2\n3\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVaccination"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k, d, w): (usize, usize, usize, usize) = input.read();

    let t = input.read_vec::<usize>(n);

    let mut expires = t[0] + w + d;

    let mut answer = 1;

    let mut vaccinated = 0;
    for i in 0..n {
        if t[i] > expires || vaccinated == k {
            expires = t[i] + w + d;
            answer += 1;
            vaccinated = 1;
        }
        else {
            vaccinated += 1;
        }
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
