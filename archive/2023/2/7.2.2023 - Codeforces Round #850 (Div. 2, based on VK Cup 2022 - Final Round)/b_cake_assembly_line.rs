//{"name":"B. Cake Assembly Line","group":"Codeforces - Codeforces Round #850 (Div. 2, based on VK Cup 2022 - Final Round)","url":"https://codeforces.com/contest/1786/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 10 5\n65 95 165\n40 65 145\n5 2 1\n1 6 11 16 21\n4 9 14 19 24\n3 3 2\n13 22 29\n5 16 25\n4 4 1\n27 36 127 136\n35 50 141 144\n","output":"YES\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCakeAssemblyLine"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let w = input.read::<i32>();
    let h = input.read::<i32>();
    let a = input.read_vec::<i32>(n);
    let b = input.read_vec::<i32>(n);

    let mut dif = vec![0; n];
    for i in 0..n {
        dif[i] = a[i] - b[i];
    }

    dif.sort();

    if dif[n-1]-dif[0] > 2*(w-h) {
        out_line!("NO");
        return;
    }
    out_line!("YES");
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
