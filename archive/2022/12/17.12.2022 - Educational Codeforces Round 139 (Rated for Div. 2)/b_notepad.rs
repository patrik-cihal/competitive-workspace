//{"name":"B. Notepad#","group":"Codeforces - Educational Codeforces Round 139 (Rated for Div. 2)","url":"https://codeforces.com/contest/1766/problem/B","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n10\ncodeforces\n8\nlabacaba\n5\nuohhh\n16\nisthissuffixtree\n1\nx\n4\nmomo\n","output":"NO\nYES\nNO\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNotepad"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v: Vec<char> = input.read::<String>().chars().collect();

    let mut pairs = BTreeSet::new();

    if n==1 {
        out_line!("NO");
        return;
    }

    for i in 2..n {
        let pair = (v[i-1], v[i]);
        if pairs.contains(&pair) {
            out_line!("YES");
            return;
        }
        pairs.insert((v[i-2], v[i-1]));
    }
    out_line!("NO");
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
