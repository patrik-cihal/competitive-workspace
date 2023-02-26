//{"name":"B. Diverse Substrings","group":"Codeforces - Codeforces Round #833 (Div. 2)","url":"https://codeforces.com/contest/1748/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1\n7\n2\n77\n4\n1010\n5\n01100\n6\n399996\n5\n23456\n18\n789987887987998798\n","output":"1\n2\n10\n12\n10\n15\n106\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDiverseSubstrings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read::<String>().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut answer = 0;

    for i in 0..n {
        let mut u = 0;
        let mut m = 0;
        let mut count = [0; 10];
        for j in ((i-100.min(i))..(i+1)).rev() {
            count[v[j] as usize] += 1;
            if count[v[j] as usize] == 1 {
                u += 1;
            }
            if count[v[j] as usize] > 10 {
                break;
            }
            m = m.max(count[v[j] as usize]);
            if u>=m {
                answer += 1;
            }
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
