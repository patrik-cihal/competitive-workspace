//{"name":"B. Asterisk-Minor Template","group":"Codeforces - Educational Codeforces Round 144 (Rated for Div. 2)","url":"https://codeforces.com/contest/1796/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\naaab\nzzzb\ncodeforces\natcoder\ncodeforces\ntokitlx\naaaa\naaaaaa\nabcd\nabcd\nc\nf\n","output":"YES\n*b\nYES\n*co*\nNO\nYES\na*a*a*a\nYES\nabcd\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAsteriskMinorTemplate"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a: Vec<char> = input.read::<String>().chars().collect();
    let b: Vec<char> = input.read::<String>().chars().collect();

    if a[0] == b[0] {
        println!("YES");
        println!("{}*", a[0]);
        return;
    }
    else if a.last().unwrap() == b.last().unwrap() {
        println!("YES");
        println!("*{}", a.last().unwrap());
        return;
    }

    let mut a_contains = BTreeSet::new();
    
    for i in 1..a.len() {
        a_contains.insert((a[i-1], a[i]));
    }

    for i in 1..b.len() {
        if a_contains.contains(&(b[i-1], b[i])) {
            println!("YES");
            println!("*{}{}*", b[i-1], b[i]);
            return;
        }
    }


    println!("NO");





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
