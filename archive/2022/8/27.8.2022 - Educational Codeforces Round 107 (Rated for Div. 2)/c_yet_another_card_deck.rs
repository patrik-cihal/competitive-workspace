//{"name":"C. Yet Another Card Deck","group":"Codeforces - Educational Codeforces Round 107 (Rated for Div. 2)","url":"https://codeforces.com/contest/1511/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7 5\n2 1 1 4 3 3 1\n3 2 1 1 4\n","output":"5 2 3 1 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CYetAnotherCardDeck"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let q: usize = input.read();

    let mut result = vec![-1; 50];
    for i in 0..n {
        let c = input.read::<usize>()-1;
        if result[c] == -1 {
            result[c] = i as i32;
        }
    }

    for i in 0..q {
        let c = input.read::<usize>()-1;
        out!(result[c]+1, "");
        for j in 0..50 {
            if result[j]<result[c] {
                result[j] += 1;
            }
        }
        result[c] = 0;
    }
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
