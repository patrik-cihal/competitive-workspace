//{"name":"B. Luke is a foodie","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5 3\n3 8 5 6 7\n5 3\n3 10 9 8 7\n12 8\n25 3 3 17 8 6 1 16 15 25 17 23\n10 2\n1 2 3 4 5 6 7 8 9 10\n8 2\n2 4 6 8 6 4 12 14\n8 2\n2 7 8 9 6 13 21 28\n15 5\n11 4 13 23 7 10 5 21 20 11 17 5 29 16 11\n","output":"0\n1\n2\n1\n2\n4\n6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLukeIsAFoodie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let x: i32 = input.read();

    let v: Vec<i32> = input.read_vec(n);
    let mut result = 0;
    let mut l = v[0];
    let mut h = v[0];
    for i in 1..n {
        l = l.min(v[i]);
        h = h.max(v[i]);
        if (h-l+1)/2 > x {
            result += 1;
            l = v[i];
            h = v[i];
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
