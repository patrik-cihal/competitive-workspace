//{"name":"D. Color with Occurrences","group":"Codeforces - Codeforces Round #811 (Div. 3)","url":"https://codeforces.com/problemset/problem/1714/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\nbababa\n2\nba\naba\ncaba\n2\nbac\nacab\nabacabaca\n3\naba\nbac\naca\nbaca\n3\na\nc\nb\ncodeforces\n4\ndef\ncode\nefo\nforces\naaaabbbbcccceeee\n4\neeee\ncccc\naaaa\nbbbb\n","output":"3\n2 2\n1 1\n2 4\n-1\n4\n1 1\n2 6\n3 3\n3 7\n4\n3 1\n1 2\n2 3\n1 4\n2\n4 5\n2 1\n4\n3 1\n4 5\n2 9\n1 13\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DColorWithOccurrences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: String = input.read(); 
    out_line!(n);
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
