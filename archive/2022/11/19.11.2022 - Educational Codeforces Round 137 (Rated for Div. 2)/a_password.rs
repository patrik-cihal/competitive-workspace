//{"name":"A. Password","group":"Codeforces - Educational Codeforces Round 137 (Rated for Div. 2)","url":"https://codeforces.com/contest/1743/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n8\n0 1 2 4 5 6 8 9\n1\n8\n","output":"6\n216\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APassword"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};



fn solve(input: &mut Input, _test_case: usize) {

    let n: usize = input.read();
    let v = input.read_vec::<i32>(n);

    let satisfies = move |num: i32| {
        let a = num%10;
        let b = num/10%10;
        let c = num/100%10;
        let d = num/1000%10;
        let mut k = vec![a, b, c, d];
        k.sort();

        k[0] == k[1] && k[2] == k[3] && k[0] != k[2] && !v.contains(&a) && !v.contains(&b) && !v.contains(&c) && !v.contains(&d)
    };

    let mut result = 0;
    for i in 0..10000 {
        if satisfies(i) {
            result += 1;
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
