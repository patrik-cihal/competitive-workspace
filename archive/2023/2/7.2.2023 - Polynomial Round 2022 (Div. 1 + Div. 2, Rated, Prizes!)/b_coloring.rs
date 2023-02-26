//{"name":"B. Coloring","group":"Codeforces - Polynomial Round 2022 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1774/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n12 6 2\n1 1 1 1 1 7\n12 6 2\n2 2 2 2 2 2\n","output":"NO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BColoring"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let m = input.read::<usize>();
    let k = input.read::<usize>();
    let a = input.read_vec::<i32>(m);

    let mx = a.iter().max().unwrap();

    let count = a.iter().filter(|x| x== &mx).count();

    if *mx == (n/k+1) as i32 {
        let overlap = n%k;
        if count<=overlap {
            out_line!("YES");
        }
        else {
            out_line!("NO");
        }
    }
    else if *mx<(n/k+1) as i32 {
        out_line!("YES");
    }
    else {
        out_line!("NO");
    }

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
